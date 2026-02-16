import { Stack } from "expo-router";
import "../global.css";
import { SafeAreaProvider } from "react-native-safe-area-context";
import { Provider } from "react-redux";
import { store } from "@/store";
import { useRestoreAuth } from "@/hooks/useRestoreAuth";

function AppContent() {
    useRestoreAuth();

    return <Stack screenOptions={{ headerShown: false }} />;
}

export default function RootLayout() {
    return (
        <Provider store={store}>
            <SafeAreaProvider>
                <AppContent />
            </SafeAreaProvider>
        </Provider>
    );
}
