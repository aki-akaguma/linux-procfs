#!/bin/sh

DEST="fixtures-1"

mkdir -p "$DEST" 2>/dev/null

for a in /proc/*/statm; do
  d=${a%/statm};
  [ "$d" = "/proc/self" -o "$d" = "/proc/thread-self" ] && continue
  #echo "$d"
  mkdir -p "$DEST$d" 2>/dev/null
  cp -a "$d/statm" "$DEST$d"
  cp -a "$d/stat" "$DEST$d"
  cp -a "$d/status" "$DEST$d"
  cp -a "$d/cmdline" "$DEST$d"
  cp -a "$d/comm" "$DEST$d"
done

mkdir -p "$DEST/proc/net" 2>/dev/null
cp -a "/proc/net/dev" "$DEST/proc/net/"

cp -a "/proc/diskstats" "$DEST/proc/"
cp -a "/proc/loadavg" "$DEST/proc/"
cp -a "/proc/meminfo" "$DEST/proc/"
cp -a "/proc/stat" "$DEST/proc/"
cp -a "/proc/uptime" "$DEST/proc/"
cp -a "/proc/vmstat" "$DEST/proc/"

mkdir -p "$DEST/sys/devices/system/cpu" 2>/dev/null
for a in /sys/devices/system/cpu/cpu*/cpufreq; do
  d=${a%/cpufreq};
  echo "$d"
done
cp -a "/sys/devices/system/cpu" "$DEST/sys/devices/system/"

