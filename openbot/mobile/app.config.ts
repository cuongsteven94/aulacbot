import type { ExpoConfig } from "expo/config";

const projectId = process.env.EXPO_PUBLIC_EAS_PROJECT_ID;
const config: ExpoConfig = {
  name: "AuLacBot",
  slug: "aulacbot",
  version: "0.0.1",
  scheme: "aulacbotmobile",
  ios: { bundleIdentifier: "app.aulacbot.mobile", supportsTablet: true },
  android: { package: "app.aulacbot.mobile" },
  plugins: [
    "expo-router",
    "expo-secure-store",
    ["expo-notifications", { defaultChannel: "aulacbot" }],
  ],
  extra: { ...(projectId ? { eas: { projectId } } : {}) },
};
export default config;
