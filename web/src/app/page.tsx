import { MessageBubble } from "@/components/message-bubble";
import { StoryPanel } from "@/components/story-panel";
import { StickerPreview } from "@/components/sticker-preview";
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
  const isoTimeMatch = value.match(/T(\d{2}:\d{2})/);
  if (isoTimeMatch) return isoTimeMatch[1];
  const timeMatch = value.match(/(\d{2}:\d{2})/);
  return timeMatch ? timeMatch[1] : value;
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
  return formatMinutesDuration(totalMinutes);
}

function formatMinutesDuration(totalMinutes: number) {
  const safeMinutes = Math.max(0, Math.round(totalMinutes));
  return `${formatNumber(safeMinutes)} минут`;
}

function estimateTypingMinutes(charCount: number) {
  const charsPerMinute = 80;
  return Math.ceil(charCount / charsPerMinute);
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

  const directPath = fileName.includes("/")
    ? path.join(sourceDir, fileName)
    : null;

  const candidates = [
    directPath,
    ...["stickers", "video_files", "files", "photos"].map((dir) =>
      path.join(sourceDir, dir, fileName)
    ),
  ].filter(Boolean) as string[];

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
  const sourceDir = process.env.STATS_SOURCE_DIR ?? data.source_dir;

  const topStickerCount = Math.max(
    stickers.owner_most_used_sticker_count,
    stickers.member_most_used_sticker_count
  );
  const topSticker =
    stickers.owner_most_used_sticker_count >= stickers.member_most_used_sticker_count
      ? stickers.owner_most_used_sticker
      : stickers.member_most_used_sticker;
  const stickerMedia = await findStickerMedia(
    sourceDir,
    topSticker?.file ?? null
  );
  const ownerStickerMedia = await findStickerMedia(
    sourceDir,
    stickers.owner_most_used_sticker?.file ??
    null
  );
  const memberStickerMedia = await findStickerMedia(
    sourceDir,
    stickers.member_most_used_sticker?.file ??
    null
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
    longestCallTime: formatTime(calls.longest_call_durations_min?.date ?? null),
    loveTotalCount: formatNumber(occurrences.total_messages_count),
    loveYouCount: formatNumber(occurrences.owner_messages_count),
    loveMeCount: formatNumber(occurrences.member_messages_count),
    firstMessageText: messageStats.first_message?.text ?? "С новым годом!",
    firstMessageTime: formatTime(messageStats.first_message?.date ?? null),
    longestChatText: conversation.first_message?.text ?? "как дела?",
    longestChatTime: formatTime(conversation.first_message?.date ?? null),
    avgMessagesPerDay: formatFloat(data.avg_messages_per_day),
    topEmoji: emojiStats.top_emoji ?? null,
    topEmojiCount: formatNumber(emojiStats.top_emoji_count),
    topWords: wordStats.top_words,
    topStickerCount: formatNumber(topStickerCount),
    stickerMedia,
    ownerStickerCount: formatNumber(stickers.owner_most_used_sticker_count),
    memberStickerCount: formatNumber(stickers.member_most_used_sticker_count),
    ownerStickerMedia,
    memberStickerMedia,
    youTypingDuration: formatMinutesDuration(
      estimateTypingMinutes(additionalStats.member_characters_count)
    ),
  };

  return (
    <div className="relative min-h-screen overflow-hidden text-foreground aurora-bg">
      <div className="pointer-events-none absolute inset-0 sparkle-field" />
      <div className="pointer-events-none absolute inset-0 soft-vignette" />
      <div className="pointer-events-none absolute -left-24 top-10 h-72 w-72 rounded-full bg-amber-200/40 blur-3xl glow-orb animate-float float-slow" />
      <div className="pointer-events-none absolute right-[-120px] top-32 h-96 w-96 rounded-full bg-rose-400/40 blur-[120px] glow-orb animate-float float-mid" />
      <div className="pointer-events-none absolute bottom-[-120px] left-20 h-80 w-80 rounded-full bg-emerald-300/30 blur-[120px] glow-orb animate-float float-fast" />

      <main className="relative mx-auto flex max-w-6xl flex-col gap-10 px-6 pb-24 pt-12">
        <header className="flex flex-col gap-3 text-center text-white/90">
          <Badge className="mx-auto w-fit rounded-full border border-white/20 bg-white/10 px-4 py-1 text-xs uppercase tracking-[0.3em] text-white">
            Telegram recap {stats.year}
          </Badge>
          <h1 className="font-display text-3xl uppercase tracking-[0.12em] headline-shine sm:text-4xl">
            Новогодние итоги года
          </h1>
        </header>

        <section className="story-grid grid gap-12 lg:grid-cols-2 lg:gap-14">
          <StoryPanel className="panel-gold panel-card">
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

          <StoryPanel className="justify-between panel-warm panel-card">
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
                {stats.youChars} символов, что заняло бы {stats.youTypingDuration}.
              </p>
              <p className="text-lg font-semibold text-white">У тебя лапки не устали?</p>
            </div>
          </StoryPanel>

          <StoryPanel className="items-center text-center panel-icy panel-card">
            <div className="absolute left-0 right-0 top-0 h-14 garland opacity-90" />
            <div className="pointer-events-none absolute inset-0 z-20 overflow-hidden">
              <div className="snowflake-field">
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
                <span className="snowflake">❄️</span>
              </div>
            </div>
            <div className="relative z-10 flex h-full flex-col items-center justify-center gap-4 pt-12 text-white">
              <div className="text-7xl drop-shadow-[0_14px_30px_rgba(220,120,0,0.6)]">
                🔥
              </div>
              <div className="font-display text-6xl">{stats.streakDays}</div>
              <p className="max-w-[260px] text-base text-white/85">
                За этот год наша серия ни разу не прервалась — мы не забывали
                друг о друге ни на день.
              </p>
              <p className="text-sm text-white/75">Маленькое чудо каждый день.</p>
            </div>
          </StoryPanel>

          <StoryPanel className="panel-rose panel-card">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="relative z-10 flex h-full flex-col justify-between pt-12 text-white">
              <div className="space-y-4">
                <Badge className="w-fit rounded-full border border-white/20 bg-white/10 px-3 py-1 text-xs uppercase tracking-[0.2em] text-white">
                  Самый долгий чат
                </Badge>
                <p className="flex items-center justify-between gap-3 text-base text-white/85">
                  <span>
                    Мы много общались, а самый длинный разговор был {stats.longestChatDate}.
                  </span>
                  <span className="text-3xl opacity-80">💬</span>
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
                <p className="text-sm text-white/75">Этот день — в копилку теплых.</p>
              </div>
            </div>
          </StoryPanel>

          <StoryPanel className="justify-between panel-gold panel-card">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="relative z-10 space-y-4 pt-12 text-white">
              <p className="text-base text-white/85">
                В этом году мы не только писали, но и разговаривали по телефону
                — {stats.callMinutes} минут за год.
              </p>
              <Card className="w-full max-w-[300px] rounded-[28px] border-0 bg-emerald-900/35 px-4 py-4 text-emerald-50 shadow-[0_18px_40px_rgba(8,22,12,0.35)]">
                <div className="relative flex items-center justify-center rounded-2xl bg-transparent px-3 py-2 pr-16">
                  <div className="text-center">
                    <div className="text-[10px] uppercase tracking-[0.24em] text-emerald-100/80">
                      Самый длинный
                    </div>
                    <div className="text-sm font-semibold text-emerald-50">
                      звонок
                    </div>
                  </div>
                  <div className="absolute right-3 rounded-full bg-emerald-200/20 px-3 py-1 text-xs text-emerald-50">
                    {stats.longestCallTime}
                  </div>
                </div>
                <div className="flex flex-col items-center gap-3 rounded-2xl bg-transparent px-4 py-4 text-center">
                  <div className="w-[156px] aspect-square">
                    <img
                      alt="Стикер звонка"
                      className="w-full h-full object-contain animate-pulse"
                      src="/sticker (321).webp"
                    />
                  </div>
                  <div>
                    <div className="text-[10px] uppercase tracking-[0.2em] text-emerald-100/80">
                      Длительность
                    </div>
                    <div className="mt-1 text-lg font-semibold">
                      {stats.longestCallMinutes} мин
                    </div>
                  </div>
                </div>
                <div className="mt-4 flex items-center justify-between gap-3">
                  <div className="flex h-12 w-12 items-center justify-center rounded-full bg-rose-500 text-white shadow-[0_10px_20px_rgba(180,40,40,0.35)]">
                    ✕
                  </div>
                  <div className="text-[10px] uppercase tracking-[0.28em] text-emerald-100/80">
                    Входящий звонок
                  </div>
                  <div className="flex h-12 w-12 items-center justify-center rounded-full bg-emerald-600 text-white shadow-[0_10px_20px_rgba(30,120,70,0.35)]">
                    ✓
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

          <StoryPanel className="justify-between panel-icy panel-card">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="absolute right-8 top-[128px] mb-2.5 text-4xl opacity-80 animate-pulse">✨</div>
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
            </div>
          </StoryPanel>

          <StoryPanel className="justify-between panel-rose panel-card">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="relative z-10 space-y-6 pt-12 text-white">
              <Badge className="w-fit rounded-full border border-white/20 bg-white/10 px-3 py-1 text-xs uppercase tracking-[0.2em] text-white">
                Стикеры года
              </Badge>
              <div className="space-y-4">
                <div className="rounded-3xl border border-white/15 bg-white/10 px-4 py-4 text-center">
                  <div className="text-xs uppercase text-white/70">Мой топ</div>
                  <div className="text-lg font-semibold">{stats.ownerStickerCount} раз</div>
                  <div className="mt-3 flex justify-center">
                    <StickerPreview media={stats.ownerStickerMedia} />
                  </div>
                </div>
                <div className="rounded-3xl border border-white/15 bg-white/10 px-4 py-4 text-center">
                  <div className="text-xs uppercase text-white/70">Твой топ</div>
                  <div className="text-lg font-semibold">{stats.memberStickerCount} раз</div>
                  <div className="mt-3 flex justify-center">
                    <StickerPreview media={stats.memberStickerMedia} />
                  </div>
                  <p className="text-xs text-white/50">(Злоба присутствует)</p>
                </div>
              </div>
            </div>
          </StoryPanel>

          <StoryPanel className="items-center text-center panel-rose panel-card">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="relative z-10 flex h-full flex-col items-center justify-center gap-6 pt-12 text-white">
              <h2 className="font-display text-3xl uppercase tracking-[0.18em]">
                Среднее в день
              </h2>
              <div className="text-5xl font-semibold">{stats.avgMessagesPerDay}</div>
              <p className="max-w-[260px] text-base text-white/85">
                сообщений в среднем за день
              </p>
            </div>
            <div className="text-left">
              <p className="text-lg font-semibold text-white">
                То что дарит улыбку каждый день
              </p>
            </div>

          </StoryPanel>


          <StoryPanel className="items-center text-center panel-warm panel-card">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="absolute bottom-6 right-6 text-5xl animate-float">❤️</div>
            <div className="relative z-10 flex h-full flex-col items-center justify-center gap-6 pt-12 text-white">
              <h2 className="font-display text-3xl uppercase tracking-[0.18em]">
                Я люблю тебя
              </h2>
              <p className="max-w-[280px] text-base text-white/85">
                Я упел признаться тебе в этом {stats.loveYouCount} раз, в прошедшем году.
              </p>
              <div className="flex items-center gap-3 text-4xl animate-float">
                💞💗
              </div>
              <p className="text-lg font-semibold text-white">
                И всё это — бесконечно мало по сравнению с тем, как сильно я тебя люблю. Спасибо тебе за ещё один год, моя любовь.
              </p>
            </div>
          </StoryPanel>
        </section>
      </main>
    </div>
  );
}
