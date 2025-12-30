type StickerMedia = {
  url: string;
  isVideo: boolean;
} | null;

export function StickerPreview({ media }: { media: StickerMedia }) {
  const sizeStyle: React.CSSProperties = {
    width: "clamp(120px, 44vw, 192px)",
    height: "clamp(120px, 44vw, 192px)",
  };

  if (!media) {
    return (
      <div
        className="flex items-center justify-center rounded-lg bg-white/10 text-[9px] text-white/70"
        style={sizeStyle}
      >
        нет файла
      </div>
    );
  }

  if (media.isVideo) {
    return (
      <video
        src={media.url}
        className="rounded-lg bg-white/10 object-contain"
        style={sizeStyle}
        autoPlay
        loop
        muted
        playsInline
      />
    );
  }

  return (
    <img
      src={media.url}
      alt="Самый частый стикер"
      className="rounded-lg bg-white/10 object-contain"
      style={sizeStyle}
    />
  );
}
