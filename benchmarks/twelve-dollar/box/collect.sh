#!/bin/sh
set -eu

out=""
pid=""
port=""
db=""
pidfile=""
interval=1

while [ $# -gt 0 ]; do
  case "$1" in
    --out)
      out="${2:?--out needs a path}"
      shift
      ;;
    --out=*)
      out="${1#*=}"
      ;;
    --pid)
      pid="${2:?--pid needs a process id}"
      shift
      ;;
    --pid=*)
      pid="${1#*=}"
      ;;
    --port)
      port="${2:?--port needs a port}"
      shift
      ;;
    --port=*)
      port="${1#*=}"
      ;;
    --db)
      db="${2:?--db needs a path}"
      shift
      ;;
    --db=*)
      db="${1#*=}"
      ;;
    --pidfile)
      pidfile="${2:?--pidfile needs a path}"
      shift
      ;;
    --pidfile=*)
      pidfile="${1#*=}"
      ;;
    --interval)
      interval="${2:?--interval needs seconds}"
      shift
      ;;
    --interval=*)
      interval="${1#*=}"
      ;;
    *)
      printf 'unknown argument: %s\n' "$1" >&2
      exit 2
      ;;
  esac
  shift
done

[ -n "$out" ] || { printf '%s\n' '--out is required' >&2; exit 2; }
[ -n "$pid" ] || { printf '%s\n' '--pid is required' >&2; exit 2; }
[ -n "$port" ] || { printf '%s\n' '--port is required' >&2; exit 2; }
[ -n "$db" ] || { printf '%s\n' '--db is required' >&2; exit 2; }

mkdir -p "$(dirname "$out")"
if [ -n "$pidfile" ]; then
  mkdir -p "$(dirname "$pidfile")"
  printf '%s\n' "$$" > "$pidfile"
fi

clk_tck="$(getconf CLK_TCK)"
start_ms="$(date +%s%3N)"
sample_index=0

header="sample,timestamp_ms,elapsed_s,clk_tck,app_pid,app_cpu_ticks,nginx_pids,nginx_cpu_ticks,app_rss_kib,app_hwm_kib,app_fds,established,db_bytes,wal_bytes,memory_current_bytes,memory_peak_bytes,cpu_usage_usec,cpu_user_usec,cpu_system_usec,cpu_nr_periods,cpu_nr_throttled,cpu_throttled_usec,pids_current"
printf '%s\n' "$header" > "$out"

read_ticks() {
  read_ticks_target="$1"
  [ -r "/proc/$read_ticks_target/stat" ] || return 0
  read_ticks_stat="$(cat "/proc/$read_ticks_target/stat" 2>/dev/null || true)"
  [ -n "$read_ticks_stat" ] || return 0
  read_ticks_rest="${read_ticks_stat##*) }"
  read_ticks_cpu_ticks="$(printf '%s\n' "$read_ticks_rest" | awk '{ print $12 + $13 }')"
  printf '%s' "${read_ticks_cpu_ticks:-0}"
}

nginx_ticks() {
  nginx_ticks_total=0
  nginx_ticks_any=0
  nginx_ticks_pids="$(pgrep -x nginx 2>/dev/null || true)"
  set -f
  for nginx_ticks_pid in $nginx_ticks_pids; do
    nginx_ticks_value="$(read_ticks "$nginx_ticks_pid")"
    if [ -n "$nginx_ticks_value" ]; then
      nginx_ticks_total=$((nginx_ticks_total + nginx_ticks_value))
      nginx_ticks_any=1
    fi
  done
  set +f
  printf '%s' "$nginx_ticks_total"
  [ "$nginx_ticks_any" -eq 1 ] || true
}

nginx_pid_list() {
  nginx_pid_list_pids="$(pgrep -x nginx 2>/dev/null || true)"
  awk 'NF { if (count++) printf "+"; printf "%s", $0 } END { if (count) printf "\n" }' <<EOF
$nginx_pid_list_pids
EOF
}

status_field() {
  status_field_name="$1"
  status_field_value="$(awk -v want="$status_field_name:" '$1 == want { print $2; exit }' "/proc/$pid/status" 2>/dev/null || true)"
  printf '%s' "$status_field_value"
}

fd_count() {
  fd_count_entries="$(find "/proc/$pid/fd" -type l -print 2>/dev/null || true)"
  fd_count_value="$(awk 'NF { count++ } END { print count + 0 }' <<EOF
$fd_count_entries
EOF
)"
  printf '%s' "${fd_count_value:-0}"
}

file_bytes() {
  file_bytes_value="$( (wc -c < "$1") 2>/dev/null || true)"
  file_bytes_value="${file_bytes_value#"${file_bytes_value%%[! ]*}"}"
  printf '%s' "${file_bytes_value:-0}"
}

cgroup_value() {
  cat "$1" 2>/dev/null || true
}

cpu_stat_field() {
  cpu_stat_field_key="$1"
  cpu_stat_field_stat="$2"
  printf '%s' "$cpu_stat_field_stat" | awk -v want="$cpu_stat_field_key" '$1 == want { print $2; exit }'
}

sample() {
  sample_now_ms="$(date +%s%3N)"
  sample_elapsed="$(awk -v now="$sample_now_ms" -v start="$start_ms" 'BEGIN { printf "%.3f", (now - start) / 1000 }')"
  sample_app_ticks="$(read_ticks "$pid")"
  sample_rss="$(status_field VmRSS)"
  sample_hwm="$(status_field VmHWM)"
  sample_fds="$(fd_count)"
  sample_connections="$(ss -H -tn state established "sport = :$port" 2>/dev/null || true)"
  sample_established="$(awk 'NF { count++ } END { print count + 0 }' <<EOF
$sample_connections
EOF
)"
  sample_db_bytes="$(file_bytes "$db")"
  sample_wal_bytes="$(file_bytes "$db-wal")"
  sample_mem_current="$(cgroup_value /sys/fs/cgroup/memory.current)"
  sample_mem_peak="$(cgroup_value /sys/fs/cgroup/memory.peak)"
  sample_pids_current="$(cgroup_value /sys/fs/cgroup/pids.current)"
  sample_cpu_stat="$(cgroup_value /sys/fs/cgroup/cpu.stat)"
  sample_usage="$(cpu_stat_field usage_usec "$sample_cpu_stat")"
  sample_user="$(cpu_stat_field user_usec "$sample_cpu_stat")"
  sample_system="$(cpu_stat_field system_usec "$sample_cpu_stat")"
  sample_periods="$(cpu_stat_field nr_periods "$sample_cpu_stat")"
  sample_throttled="$(cpu_stat_field nr_throttled "$sample_cpu_stat")"
  sample_throttled_usec="$(cpu_stat_field throttled_usec "$sample_cpu_stat")"
  sample_nginx_pid_list="$(nginx_pid_list)"
  sample_nginx_ticks="$(nginx_ticks)"

  printf '%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s\n' \
    "$sample_index" "$sample_now_ms" "$sample_elapsed" "$clk_tck" "$pid" "$sample_app_ticks" \
    "$sample_nginx_pid_list" "$sample_nginx_ticks" "$sample_rss" "$sample_hwm" "$sample_fds" "$sample_established" \
    "$sample_db_bytes" "$sample_wal_bytes" "$sample_mem_current" "$sample_mem_peak" \
    "$sample_usage" "$sample_user" "$sample_system" "$sample_periods" "$sample_throttled" "$sample_throttled_usec" "$sample_pids_current" \
    >> "$out"
  sample_index=$((sample_index + 1))
}

finish() {
  sample || true
  exit 0
}

trap finish TERM INT

while :; do
  sample || true
  sleep "$interval"
done
