import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  allowedDevOrigins: [
    "192.168.0.189",
    "192.168.0.210",
    "192.168.0.145",
    "localhost",
    "127.0.0.1",
  ],
};

export default nextConfig;
