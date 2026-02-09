import { View } from "react-native";
import { Slot } from "expo-router";
import { AuthTabs } from "../../components/auth/AuthTabs";

export default function AuthLayout() {
    return (
        <View className="flex-1">
            <Slot />
            <AuthTabs />
        </View>
    );
}
