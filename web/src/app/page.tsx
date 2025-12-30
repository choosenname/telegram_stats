import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { cn } from "@/lib/utils";
import type { ReactNode } from "react";
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
  chat_stats: {
    messages_stats: MessagesStats;
    additional_messages_stats: AdditionalMessagesStats;
  };
  occurrences: OccurrenceStats;
  longest_conversation: LongestConversationStats;
  calls_stats: CallsStats;
  most_used_sticker: unknown;
  streak: {
    count: number;
    start: string;
    end: string;
  };
};

function formatNumber(value: number) {
  return new Intl.NumberFormat("ru-RU").format(value);
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

export const dynamic = "force-dynamic";

export default async function Home() {
  const data = await getStats();
  const messageStats = data.chat_stats.messages_stats;
  const additionalStats = data.chat_stats.additional_messages_stats;
  const conversation = data.longest_conversation;
  const occurrences = data.occurrences;
  const calls = data.calls_stats;
  const streak = data.streak;

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
  };

  return (
    <div className="min-h-screen bg-[radial-gradient(circle_at_top,_#1b5c34_0%,_#0a2e18_55%,_#071a10_100%)] text-foreground">
      <main className="mx-auto flex max-w-6xl flex-col gap-10 px-6 pb-24 pt-12">
        <header className="flex flex-col gap-3 text-center text-white/90">
          <Badge className="mx-auto w-fit rounded-full border border-white/20 bg-white/10 px-4 py-1 text-xs uppercase tracking-[0.3em] text-white">
            Telegram recap {stats.year}
          </Badge>
          <h1 className="font-display text-3xl uppercase tracking-[0.12em] text-white drop-shadow-sm sm:text-4xl">
            Новогодняя история переписки
          </h1>
          <p className="mx-auto max-w-2xl text-base text-white/70">
            Шесть праздничных карточек, вдохновленных итогами года, как в
            сторис Telegram.
          </p>
        </header>

        <section className="grid gap-12 lg:grid-cols-2 lg:gap-14">
          <StoryPanel>
            <div className="absolute left-0 right-0 top-0 h-14 garland opacity-90" />
            <div className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-white/90 via-white/60 to-transparent" />
            <div className="relative z-10 flex h-full flex-col gap-6 pt-8 text-white">
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

          <StoryPanel className="justify-between">
            <div className="absolute right-6 top-8 h-24 w-24 rounded-full bg-gradient-to-b from-amber-200 via-amber-400 to-amber-600 shadow-[0_18px_40px_rgba(60,30,10,0.45)] animate-float" />
            <div className="relative z-10 flex flex-col gap-4 pt-10 text-white">
              <p className="text-base text-white/85">
                Самым первым сообщением в этом году было мое поздравление.
              </p>
              <div className="tg-bubble w-fit max-w-[260px] bg-[#e6f7cf] px-4 py-2 text-sm text-emerald-950">
                <div className="font-medium">{stats.firstMessageText}</div>
                <div className="mt-1 flex items-center justify-end gap-1 text-[10px] text-emerald-700">
                  <span>{stats.firstMessageTime}</span>
                  <span className="tracking-[-0.08em]">✓✓</span>
                </div>
              </div>
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

          <StoryPanel className="items-center text-center">
            <div className="absolute left-0 right-0 top-0 h-14 garland opacity-90" />
            <div className="relative z-10 flex h-full flex-col items-center justify-center gap-4 pt-8 text-white">
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

          <StoryPanel>
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="relative z-10 flex h-full flex-col justify-between pt-8 text-white">
              <div className="space-y-4">
                <Badge className="w-fit rounded-full border border-white/20 bg-white/10 px-3 py-1 text-xs uppercase tracking-[0.2em] text-white">
                  Самый долгий чат
                </Badge>
                <p className="text-base text-white/85">
                  Мы много общались, а самый длинный разговор был {stats.longestChatDate}.
                </p>
                <div className="tg-bubble w-fit max-w-[260px] bg-[#e6f7cf] px-4 py-2 text-sm text-emerald-950">
                  <div className="font-medium">{stats.longestChatText}</div>
                  <div className="mt-1 flex items-center justify-end gap-1 text-[10px] text-emerald-700">
                    <span>{stats.longestChatTime}</span>
                    <span className="tracking-[-0.08em]">✓✓</span>
                  </div>
                </div>
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

          <StoryPanel className="justify-between">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="relative z-10 space-y-4 pt-8 text-white">
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

          <StoryPanel className="items-center text-center">
            <div className="absolute inset-x-0 top-0 h-14 garland opacity-90" />
            <div className="absolute bottom-6 right-6 text-5xl animate-float">❤️</div>
            <div className="relative z-10 flex h-full flex-col items-center justify-center gap-6 pt-10 text-white">
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

function StoryPanel({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) {
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
