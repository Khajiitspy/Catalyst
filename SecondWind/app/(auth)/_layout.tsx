import {View, KeyboardAvoidingView, Platform, ScrollView} from "react-native";
import {Slot} from "expo-router";
import {AuthTabs} from "../../components/auth/AuthTabs";
import {SafeAreaView} from "react-native-safe-area-context";

export default function AuthLayout() {
    return (
        <View className="flex-1 bg-white dark:bg-zinc-950">

            <SafeAreaView className="flex-1 justify-between">

                {/*<ScrollView*/}
                {/*    keyboardShouldPersistTaps="handled"*/}
                {/*    contentContainerStyle={{paddingBottom: 10}}*/}
                {/*    showsVerticalScrollIndicator={false}*/}
                {/*>*/}
                    <KeyboardAvoidingView
                        behavior={Platform.OS === "ios" ? "padding" : undefined}
                        className="flex-1"
                    >
                        <Slot/>
                    </KeyboardAvoidingView>

                    <AuthTabs/>
                {/*</ScrollView>*/}
            </SafeAreaView>
        </View>
    );
}
