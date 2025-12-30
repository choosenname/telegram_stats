import { cn } from "@/lib/utils";

type MessageBubbleProps = {
  text: string;
  time: string;
  incoming?: boolean;
};

export function MessageBubble({ text, time, incoming = false }: MessageBubbleProps) {
  const bubbleStyle: React.CSSProperties = {
    borderRadius: 24,
    borderBottomRightRadius: incoming ? 24 : 10,
    borderBottomLeftRadius: incoming ? 10 : 24,
  };

  return (
    <div
      className={cn(
        "tg-bubble w-fit max-w-[260px] bg-[#e6f7cf] px-4 py-2 text-sm text-emerald-950",
        incoming ? "incoming" : "self-end"
      )}
      style={bubbleStyle}
    >
      <div className="font-medium">{text}</div>
      <div className="mt-1 flex items-center justify-end gap-1 text-[10px] text-emerald-700 text-right">
        <span className="ml-auto">{time}</span>
        {!incoming && <span className="tracking-[-0.08em]">✓</span>}
      </div>
    </div>
  );
}
