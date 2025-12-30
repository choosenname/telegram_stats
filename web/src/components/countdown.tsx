"use client";

import { useEffect, useMemo, useState } from "react";

type CountdownProps = {
  targetIso: string;
};

function formatUnit(value: number) {
  return String(value).padStart(2, "0");
}

export function Countdown({ targetIso }: CountdownProps) {
  const targetMs = useMemo(() => Date.parse(targetIso), [targetIso]);
  const [remainingMs, setRemainingMs] = useState(() =>
    Math.max(0, targetMs - Date.now())
  );

  useEffect(() => {
    const tick = () => {
      setRemainingMs(Math.max(0, targetMs - Date.now()));
    };
    tick();
    const intervalId = window.setInterval(tick, 1000);
    return () => window.clearInterval(intervalId);
  }, [targetMs]);

  const totalSeconds = Math.floor(remainingMs / 1000);
  const days = Math.floor(totalSeconds / 86400);
  const hours = Math.floor((totalSeconds % 86400) / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  return (
    <div className="flex flex-wrap items-center justify-center gap-3 text-white/90">
      <div className="rounded-full border border-white/15 bg-white/10 px-4 py-2 text-sm uppercase tracking-[0.22em]">
        {days} д
      </div>
      <div className="rounded-full border border-white/15 bg-white/10 px-4 py-2 text-sm uppercase tracking-[0.22em]">
        {formatUnit(hours)} ч
      </div>
      <div className="rounded-full border border-white/15 bg-white/10 px-4 py-2 text-sm uppercase tracking-[0.22em]">
        {formatUnit(minutes)} м
      </div>
      <div className="rounded-full border border-white/15 bg-white/10 px-4 py-2 text-sm uppercase tracking-[0.22em]">
        {formatUnit(seconds)} с
      </div>
    </div>
  );
}
