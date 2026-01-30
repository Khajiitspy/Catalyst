import { useEffect } from "react";
import { useColorScheme } from "react-native";
import * as NavigationBar from "expo-navigation-bar";

export function useSystemBars() {
    const scheme = useColorScheme();
    const isDark = scheme === "dark";

    useEffect(() => {
        NavigationBar.setBackgroundColorAsync(
            isDark ? "#09090b" : "#fafafa"
        );
        NavigationBar.setButtonStyleAsync(
            isDark ? "light" : "dark"
        );
    }, [isDark]);
}
