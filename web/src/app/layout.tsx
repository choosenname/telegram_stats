import type { Metadata } from "next";
import { Manrope, Yeseva_One } from "next/font/google";
import "./globals.css";

const yeseva = Yeseva_One({
  variable: "--font-display",
  subsets: ["latin", "cyrillic"],
  weight: ["400"],
});

const manrope = Manrope({
  variable: "--font-body",
  subsets: ["latin", "cyrillic"],
  weight: ["300", "400", "500", "600", "700"],
});

export const metadata: Metadata = {
  title: "Telegram Year Story",
  description: "Festive chat stats story",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ru">
      <body className={`${yeseva.variable} ${manrope.variable} antialiased`}>
        {children}
      </body>
    </html>
  );
}
