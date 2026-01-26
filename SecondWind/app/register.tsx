import { View, Text, TextInput, Pressable, Image } from "react-native";
import { useState } from "react";
import * as ImagePicker from "expo-image-picker";

export default function Register() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [image, setImage] = useState<string | null>(null);

  const [errors, setErrors] = useState<{
    email?: string;
    password?: string;
    confirmPassword?: string;
  }>({});

  const pickImage = async () => {
    const result = await ImagePicker.launchImageLibraryAsync({
      mediaTypes: ImagePicker.MediaTypeOptions.Images,
      allowsEditing: true,
      aspect: [1, 1],
      quality: 0.7,
    });

    if (!result.canceled) {
      setImage(result.assets[0].uri);
    }
  };

  const validate = () => {
    const newErrors: typeof errors = {};

    if (!email.trim()) {
      newErrors.email = "Email обовʼязковий";
    } else if (!/\S+@\S+\.\S+/.test(email)) {
      newErrors.email = "Невірний формат email";
    }

    if (!password) {
      newErrors.password = "Пароль обовʼязковий";
    } else if (password.length < 6) {
      newErrors.password = "Мінімум 6 символів";
    }

    if (confirmPassword !== password) {
      newErrors.confirmPassword = "Паролі не співпадають";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const [loading, setLoading] = useState(false);

  const handleRegister = async () => {
    if (!validate()) return;

    setLoading(true);

    try {
      const response = await fetch("http://192.168.40.5:3000/register", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          email,
          password,
        }),
      });

      const data = await response.json();

      if (!data.success) {
        alert(data.message);
        return;
      }

      alert("Registration successful!");
      // optionally navigate to login screen here
    } catch (err) {
      console.error(err);
      alert("Server error");
    } finally {
      setLoading(false);
    }
  };

  const isFormValid =
    email &&
    password &&
    confirmPassword &&
    password.length >= 6 &&
    password === confirmPassword &&
    /\S+@\S+\.\S+/.test(email);

  return (
    <View className="flex-1 bg-white px-6 justify-center">
      <Text className="text-2xl font-bold text-center mb-6">
        Реєстрація
      </Text>

      {/* Photo */}
      <Pressable onPress={pickImage} className="self-center mb-6">
        {image ? (
          <Image
            source={{ uri: image }}
            className="w-28 h-28 rounded-full"
          />
        ) : (
          <View className="w-28 h-28 rounded-full bg-gray-200 items-center justify-center">
            <Text className="text-gray-500">Додати фото</Text>
          </View>
        )}
      </Pressable>

      {/* Email */}
      <TextInput
        placeholder="Email"
        value={email}
        onChangeText={(text) => {
          setEmail(text);
          setErrors((e) => ({ ...e, email: undefined }));
        }}
        autoCapitalize="none"
        keyboardType="email-address"
        className="border border-gray-300 rounded-lg px-4 py-3 mb-4"
      />
      {errors.email && (
        <Text className="text-red-500 text-sm mt-1 mb-3">
          {errors.email}
        </Text>
      )}

      {/* Password */}
      <TextInput
        placeholder="Пароль"
        value={password}
        onChangeText={(text) => {
          setPassword(text);
          setErrors((e) => ({ ...e, password: undefined }));
        }}
        secureTextEntry
        className="border border-gray-300 rounded-lg px-4 py-3 mb-4"
      />
      {errors.password && (
        <Text className="text-red-500 text-sm mt-1 mb-3">
          {errors.password}
        </Text>
      )}

      {/* Confirm Password */}
      <TextInput
        placeholder="Підтвердити пароль"
        value={confirmPassword}
        onChangeText={(text) => {
          setConfirmPassword(text);
          setErrors((e) => ({ ...e, confirmPassword: undefined }));
        }}
        secureTextEntry
        className="border border-gray-300 rounded-lg px-4 py-3 mb-4"
      />
      {errors.confirmPassword && (
        <Text className="text-red-500 text-sm mt-1 mb-6">
          {errors.confirmPassword}
        </Text>
      )}

      {/* Submit */}
      <Pressable
        onPress={handleRegister}
        className={`py-4 rounded-lg mb-4 bg-blue-500`}
      >
        <Text className="text-white text-center font-semibold text-lg">
          Зареєструватися
        </Text>
      </Pressable>
    </View>
  );
}
