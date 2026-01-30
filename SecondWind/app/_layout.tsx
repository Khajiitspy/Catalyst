import { Stack } from "expo-router";
import "../global.css";
import { SafeAreaProvider } from "react-native-safe-area-context";
import {store} from "@/store";
import {Provider} from "react-redux";

export default function RootLayout() {

    return (
        <>
            <Provider store={store}>
                <SafeAreaProvider>
                    <Stack screenOptions={{ headerShown: false }} />
                </SafeAreaProvider>
            </Provider>
        </>
    );
}
