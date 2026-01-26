import "../global.css";
import { Slot } from 'expo-router';
import { Provider } from 'react-redux';

export default function Layout() {
  return (
    <Slot />
  );
}

