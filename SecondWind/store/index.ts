import { configureStore } from '@reduxjs/toolkit';
import {type TypedUseSelectorHook, useDispatch, useSelector} from 'react-redux';
import {authService} from "@/services/authService";
import {setupListeners} from "@reduxjs/toolkit/query";
import authReducer from "./slices/authSlice";
import {chatService} from "@/services/chatService";

export const store = configureStore({
    reducer: {
        auth: authReducer,
        [authService.reducerPath]: authService.reducer,
        [chatService.reducerPath]: chatService.reducer
    },

    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware().concat(
            authService.middleware,
            chatService.middleware
        )
});

setupListeners(store.dispatch);
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export const useAppDispatch: () => AppDispatch = useDispatch
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector
