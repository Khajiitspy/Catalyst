import { View, Text, Pressable } from "react-native";
import { router } from "expo-router";

export default function Index() {
  return (
    <View className="flex-1 justify-center items-center gap-6">
      <Text className="text-xl font-bold text-blue-500">
        With Love ...
      </Text>

      <Pressable
        onPress={() => router.push("/register")}
        className="bg-blue-500 px-6 py-3 rounded-lg"
      >
        <Text className="text-white font-semibold text-lg">
          Реєстрація
        </Text>
      </Pressable>
    </View>
  );
}
