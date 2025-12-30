import { MessageBubble } from "@/components/message-bubble";
import { StoryPanel } from "@/components/story-panel";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { readFile } from "fs/promises";
import path from "path";
import { unstable_noStore as noStore } from "next/cache";

type MessageSnapshot = {
  id: number;
  from: string | null;
  type: string;
  text: string;
  date: string;
  duration_seconds: number | null;
  discard_reason: string | null;
  file: string | null;
  file_name: string | null;
  media_type: string | null;
};

type MessagesStats = {
  first_message: MessageSnapshot | null;
  last_message: MessageSnapshot | null;
  total_messages_count: number;
  owner_messages_count: number;
  member_messages_count: number;
};

type AdditionalMessagesStats = {
  total_characters_count: number;
  owner_characters_count: number;
  member_characters_count: number;
};

type LongestConversationStats = {
  first_message: MessageSnapshot | null;
  last_message: MessageSnapshot | null;
  total_messages_count: number;
  owner_messages_count: number;
  member_messages_count: number;
};

type CallsStats = {
  total_calls_durations_sec: number;
  total_calls_durations_min: number;
  longest_call_durations_min: MessageSnapshot | null;
};

type OccurrenceStats = MessagesStats;

type StatsData = {
  year: number;
  source_dir: string;
  chat_stats: {
    messages_stats: MessagesStats;
    additional_messages_stats: AdditionalMessagesStats;
  };
  occurrences: OccurrenceStats;
  longest_conversation: LongestConversationStats;
  calls_stats: CallsStats;
  most_used_sticker: {
    owner_most_used_sticker_count: number;
    owner_most_used_sticker: MessageSnapshot | null;
    member_most_used_sticker_count: number;
    member_most_used_sticker: MessageSnapshot | null;
  };
  emoji_stats: {
    top_emoji: string | null;
    top_emoji_count: number;
  };
  word_stats: {
    top_words: {
      word: string;
      count: number;
    }[];
  };
  avg_messages_per_day: number;
  streak: {
    count: number;
    start: string;
    end: string;
  };
};

function formatNumber(value: number) {
  return new Intl.NumberFormat("ru-RU").format(value);
}

function formatFloat(value: number) {
  return new Intl.NumberFormat("ru-RU", { maximumFractionDigits: 1 }).format(value);
}

function formatTime(value: string | null) {
  if (!value) return "--:--";
  return new Date(value).toLocaleTimeString("ru-RU", {
    hour: "2-digit",
    minute: "2-digit",
  });
}

function formatDayMonth(value: string | null) {
  if (!value) return "";
  return new Date(value).toLocaleDateString("ru-RU", {
    day: "numeric",
    month: "long",
  });
}

function formatDuration(from: string | null, to: string | null) {
  if (!from || !to) return "";
  const diffMs = Math.max(0, new Date(to).getTime() - new Date(from).getTime());
  const totalMinutes = Math.round(diffMs / 60000);
  const hours = Math.floor(totalMinutes / 60);
  const minutes = totalMinutes % 60;
  if (hours === 0) {
    return `${minutes} минут`;
  }
  if (minutes === 0) {
    return `${hours} часов`;
  }
  return `${hours} часов ${minutes} минут`;
}

async function getStats(): Promise<StatsData> {
  noStore();
  const dataPath = path.resolve(process.cwd(), "..", "output.json");
  const raw = await readFile(dataPath, "utf-8");
  return JSON.parse(raw) as StatsData;
}

async function findStickerMedia(sourceDir: string, fileName: string | null) {
  if (!fileName) {
    return null;
  }

  const candidates = ["stickers", "video_files", "files", "photos"].map((dir) =>
    path.join(sourceDir, dir, fileName)
  );

  for (const filePath of candidates) {
    try {
      const file = await readFile(filePath);
      const ext = path.extname(filePath).toLowerCase();
      if (ext === ".tgs") {
        continue;
      }
      const mime =
        ext === ".webm"
          ? "video/webm"
          : ext === ".png"
            ? "image/png"
            : ext === ".jpg" || ext === ".jpeg"
              ? "image/jpeg"
              : "image/webp";
      return {
        url: `data:${mime};base64,${file.toString("base64")}`,
        isVideo: ext === ".webm",
      };
    } catch {
      // Try next folder.
    }
  }

  return null;
}

export const dynamic = "force-dynamic";

export default async function Home() {
  const data = await getStats();
  const messageStats = data.chat_stats.messages_stats;
  const additionalStats = data.chat_stats.additional_messages_stats;
  const conversation = data.longest_conversation;
  const occurrences = data.occurrences;
  const calls = data.calls_stats;
  const streak = data.streak;
  const stickers = data.most_used_sticker;
  const emojiStats = data.emoji_stats;
  const wordStats = data.word_stats;

  const topStickerCount = Math.max(
    stickers.owner_most_used_sticker_count,
    stickers.member_most_used_sticker_count
  );
  const topSticker =
    stickers.owner_most_used_sticker_count >= stickers.member_most_used_sticker_count
      ? stickers.owner_most_used_sticker
      : stickers.member_most_used_sticker;
  const stickerMedia = await findStickerMedia(
    data.source_dir,
    topSticker?.file ?? null
  );

  const stats = {
    year:
      data.year ??
      (messageStats.first_message?.date
        ? new Date(messageStats.first_message.date).getFullYear()
        : 0),
    totalMessages: formatNumber(messageStats.total_messages_count),
    youMessages: formatNumber(messageStats.member_messages_count),
    youChars: formatNumber(additionalStats.member_characters_count),
    streakDays: formatNumber(streak.count),
    longestChatDate: formatDayMonth(conversation.first_message?.date ?? null),
    longestChatMessages: formatNumber(conversation.total_messages_count),
    longestChatDuration: formatDuration(
      conversation.first_message?.date ?? null,
      conversation.last_message?.date ?? null
    ),
    callMinutes: formatNumber(calls.total_calls_durations_min),
    longestCallMinutes: formatNumber(
      Math.round((calls.longest_call_durations_min?.duration_seconds ?? 0) / 60)
    ),
    loveYouCount: formatNumber(occurrences.owner_messages_count),
    loveMeCount: formatNumber(occurrences.member_messages_count),
    firstMessageText: messageStats.first_message?.text ?? "С новым годом!",
    firstMessageTime: formatTime(messageStats.first_message?.date ?? null),
    longestChatText: conversation.first_message?.text ?? "как дела?",
    longestChatTime: formatTime(conversation.first_message?.date ?? null),
    avgMessagesPerDay: formatFloat(data.avg_messages_per_day),
    topEmoji: emojiStats.top_emoji ?? "✨",
    topEmojiCount: formatNumber(emojiStats.top_emoji_count),
    topWords: wordStats.top_words,
    topStickerCount: formatNumber(topStickerCount),
    stickerMedia,
  };

  return (
    <div className="relative min-h-screen overflow-hidden text-foreground aurora-bg">
      <div className="pointer-events-none absolute inset-0 sparkle-field" />
      <div className="pointer-events-none absolute -left-24 top-10 h-72 w-72 rounded-full bg-amber-200/40 blur-3xl glow-orb animate-float" />
      <div className="pointer-events-none absolute right-[-120px] top-32 h-96 w-96 rounded-full bg-rose-400/40 blur-[120px] glow-orb animate-float" />
      <div className="pointer-events-none absolute bottom-[-120px] left-20 h-80 w-80 rounded-full bg-emerald-300/30 blur-[120px] glow-orb animate-float" />

      <main className="relative mx-auto flex max-w-6xl flex-col gap-10 px-6 pb-24 pt-12">
        <header className="flex flex-col gap-3 text-center text-white/90">
          <Badge className="mx-auto w-fit rounded-full border border-white/20 bg-white/10 px-4 py-1 text-xs uppercase tracking-[0.3em] text-white">
            Telegram recap {stats.year}
          </Badge>
          <h1 className="font-display text-3xl uppercase tracking-[0.12em] headline-shine sm:text-4xl">
            Новогоднии итоги года
          </h1>
        </header>

        <section className="grid gap-12 lg:grid-cols-2 lg:gap-14">
          <StoryPanel className="panel-gold">
            <div className="absolute left-0 right-0 top-0 h-14 garland opacity-90" />
            <div className="absolute left-6 top-10 h-16 w-16 rounded-full bg-amber-300/70 blur-lg" />
            <div className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-white/90 via-white/60 to-transparent" />
            <div className="absolute right-8 top-40 mb-2.5 text-5xl opacity-80">🎄</div>
            <div className="relative z-10 flex h-full flex-col gap-6 pt-12 text-white">
              <Badge className="w-fit rounded-full border border-white/20 bg-white/10 px-3 py-1 text-xs uppercase tracking-[0.25em] text-white">
                {stats.year}
              </Badge>
              <h2 className="font-display text-4xl uppercase leading-tight tracking-[0.12em] text-white frosted-title sm:text-5xl">
                С Новым Годом,
                <br />
                любимая!
              </h2>
              <p className="max-w-[260px] text-lg text-white/80">
                Я подготовил небольшие итоги года о нас и нашей переписке.
              </p>
            </div>
          </StoryPanel>

          <StoryPanel className="justify-between panel-warm">
            <div className="absolute right-6 top-8 h-24 w-24 rounded-full bg-gradient-to-b from-amber-200 via-amber-400 to-amber-600 shadow-[0_18px_40px_rgba(60,30,10,0.45)] animate-float" />
            <div className="absolute left-0 right-0 top-6 h-12 light-string opacity-90" />
            <div className="absolute left-6 bottom-10 text-4xl">🎁</div>
            <div className="relative z-10 flex flex-col gap-4 pt-12 text-white">
              <p className="text-base text-white/85">
                Самым первым сообщением в этом году было мое поздравление.
              </p>
              <MessageBubble text={stats.firstMessageText} time={stats.firstMessageTime} />
              <p className="text-base text-white/85">
                За год мы написали <span className="text-white">{stats.totalMessages}</span>{" "}
                сообщений.
              </p>
              <p className="text-base text-white/85">
                <span className="text-white">{stats.youMessages}</span> из них написала ты — это{" "}
                {stats.youChars} символов, что заняло бы 29 000 минут.
              </p>
              <p className="text-lg font-semibold text-white">У тебя лапки не устали?</p>
            </div>
          </StoryPanel>

          <StoryPanel className="items-center text-center panel-icy">
            <div className="absolute left-0 right-0 top-0 h-14 garland opacity-90" />
            <div className="absolute left-8 top-40 mb-2.5 text-5xl opacity-80">❄️</div>
            <div className="relative z-10 flex h-full flex-col items-center justify-center gap-4 pt-12 text-white">
              <div className="text-7xl drop-shadow-[0_14px_30px_rgba(220,120,0,0.6)]">
                🔥
              </div>
              <div className="font-display text-6xl">{stats.streakDays}</div>
              <p className="max-w-[260px] text-base text-white/85">
                За этот год наша серия ни разу не прервалась — мы не забывали
                друг о друге ни на день.
              </p>
            </div>
          </StoryPanel>

          <StoryPanel className="panel-rose">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="absolute right-8 top-32 mb-2.5 text-4xl opacity-80">💬</div>
            <div className="relative z-10 flex h-full flex-col justify-between pt-12 text-white">
              <div className="space-y-4">
                <Badge className="w-fit rounded-full border border-white/20 bg-white/10 px-3 py-1 text-xs uppercase tracking-[0.2em] text-white">
                  Самый долгий чат
                </Badge>
                <p className="text-base text-white/85">
                  Мы много общались, а самый длинный разговор был {stats.longestChatDate}.
                </p>
                <MessageBubble text={stats.longestChatText} time={stats.longestChatTime} />
              </div>
              <div className="space-y-2">
                <p className="text-base text-white/85">
                  За {stats.longestChatDuration} мы успели написать
                </p>
                <p className="font-display text-4xl text-white">
                  {stats.longestChatMessages}
                </p>
                <p className="text-base text-white/85">сообщений подряд</p>
              </div>
            </div>
          </StoryPanel>

          <StoryPanel className="justify-between panel-gold">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="absolute left-8 top-32 mb-2.5 text-4xl opacity-80">📞</div>
            <div className="relative z-10 space-y-4 pt-12 text-white">
              <p className="text-base text-white/85">
                В этом году мы не только писали, но и разговаривали по телефону
                — {stats.callMinutes} минут за год.
              </p>
              <Card className="tg-call w-full max-w-[260px] border-none px-4 py-3 !bg-[#e6f7cf] !text-emerald-950">
                <div className="flex items-center gap-3">
                  <div className="flex h-9 w-9 items-center justify-center rounded-full bg-emerald-600 text-white">
                    📞
                  </div>
                  <div className="flex-1">
                    <div className="text-xs uppercase text-emerald-700">
                      Outgoing Call
                    </div>
                    <div className="text-sm font-semibold">03:29</div>
                  </div>
                  <div className="text-[10px] text-emerald-700">
                    {stats.longestCallMinutes} мин
                  </div>
                </div>
              </Card>
            </div>
            <div className="text-left">
              <p className="text-lg font-semibold text-white">
                Я очень люблю слышать твой голос
              </p>
            </div>
          </StoryPanel>

          <StoryPanel className="justify-between panel-icy">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="absolute right-8 top-[128px] mb-2.5 text-4xl opacity-80">✨</div>
            <div className="relative z-10 space-y-5 pt-12 text-white">
              <Badge className="w-fit rounded-full border border-white/20 bg-white/10 px-3 py-1 text-xs uppercase tracking-[0.2em] text-white">
                Самые частые
              </Badge>
              <div className="space-y-3">
                <div className="text-sm uppercase text-white/70">Самый частый эмодзи</div>
                <div className="flex items-end gap-3">
                  <div className="text-5xl">{stats.topEmoji}</div>
                  <div className="text-xl font-semibold">{stats.topEmojiCount} раз</div>
                </div>
              </div>
              <div>
                <div className="text-sm uppercase text-white/70">Самые частые слова</div>
                <div className="mt-3 space-y-2">
                  {stats.topWords.map((word) => (
                    <div
                      key={word.word}
                      className="flex items-center justify-between rounded-full border border-white/20 bg-white/10 px-3 py-2 text-sm"
                    >
                      <span className="truncate">{word.word}</span>
                      <span className="text-white/80">{formatNumber(word.count)}</span>
                    </div>
                  ))}
                </div>
              </div>
              <div className="space-y-2">
                <div className="text-sm uppercase text-white/70">Самый частый стикер</div>
                {stats.stickerMedia ? (
                  <div className="flex items-center gap-3">
                    {stats.stickerMedia.isVideo ? (
                      <video
                        src={stats.stickerMedia.url}
                        className="h-16 w-16 rounded-2xl bg-white/10 object-contain"
                        autoPlay
                        loop
                        muted
                        playsInline
                      />
                    ) : (
                      <img
                        src={stats.stickerMedia.url}
                        alt="Самый частый стикер"
                        className="h-16 w-16 rounded-2xl bg-white/10 object-contain"
                      />
                    )}
                    <div className="text-lg font-semibold">{stats.topStickerCount} раз</div>
                  </div>
                ) : (
                  <div className="text-base text-white/70">
                    Файл стикера не найден, но использовался {stats.topStickerCount} раз.
                  </div>
                )}
              </div>
            </div>
          </StoryPanel>

          <StoryPanel className="items-center text-center panel-rose">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="absolute left-8 top-[128px] mb-2.5 text-4xl opacity-80">📌</div>
            <div className="relative z-10 flex h-full flex-col items-center justify-center gap-6 pt-12 text-white">
              <h2 className="font-display text-3xl uppercase tracking-[0.18em]">
                Среднее в день
              </h2>
              <div className="text-5xl font-semibold">{stats.avgMessagesPerDay}</div>
              <p className="max-w-[260px] text-base text-white/85">
                сообщений в среднем за день
              </p>
            </div>
          </StoryPanel>


          <StoryPanel className="items-center text-center panel-warm">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="absolute bottom-6 right-6 text-5xl animate-float">❤️</div>
            <div className="absolute left-8 bottom-10 text-4xl opacity-80">💖</div>
            <div className="relative z-10 flex h-full flex-col items-center justify-center gap-6 pt-12 text-white">
              <h2 className="font-display text-3xl uppercase tracking-[0.18em]">
                Я люблю тебя
              </h2>
              <p className="max-w-[280px] text-base text-white/85">
                {stats.loveYouCount} раз я говорил это в прошедшем году, а ты —{" "}
                {stats.loveMeCount} раз.
              </p>
              <div className="flex items-center gap-3 text-4xl">
                💞💗
              </div>
              <p className="max-w-[280px] text-base text-white/85">
                Я бесконечно сильно тебя люблю. Спасибо, что была со мной весь
                год.
              </p>
            </div>
          </StoryPanel>
        </section>
      </main>
    </div>
  );
}
