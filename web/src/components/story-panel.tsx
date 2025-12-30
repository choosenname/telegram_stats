import { Card } from "@/components/ui/card";
import { cn } from "@/lib/utils";

type StoryPanelProps = {
  children: React.ReactNode;
  className?: string;
};

export function StoryPanel({ children, className }: StoryPanelProps) {
  return (
    <Card
      className={cn(
        "relative flex min-h-[620px] w-full max-w-[420px] flex-col overflow-hidden border border-white/10 p-8 text-white festive-panel animate-fade-up",
        className
      )}
    >
      <div className="pointer-events-none absolute inset-0 snowfall" />
      <div className="pointer-events-none absolute inset-0 bg-gradient-to-b from-white/5 via-transparent to-black/30" />
      {children}
    </Card>
  );
}
