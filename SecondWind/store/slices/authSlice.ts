import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
import { jwtDecode } from "jwt-decode";
import { IUserIdentity } from "@/types/auth/IUserIdentity";
import storage from "@/store/storages/tokenStorage";

interface AuthState {
    user: IUserIdentity | null;
}

const getUserFromToken = (token: string): IUserIdentity | null => {
    try {
        const decoded: any = jwtDecode(token);
        return {
            name:
                decoded["name"] ??
                decoded["http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name"] ??
                "",
            email:
                decoded["email"] ??
                decoded[
                    "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress"
                    ] ??
                "",
            image: decoded["image"] ?? "",
            token,
            role: decoded["roles"] ?? null,
        };
    } catch (e) {
        console.error("Invalid token", e);
        return null;
    }
};

const initialState: AuthState = {
    user: null,
};

const authSlice = createSlice({
    name: "auth",
    initialState,
    reducers: {
        loginSuccess: (state, action: PayloadAction<string | null>) => {
            state.user = action.payload ? getUserFromToken(action.payload) : null;
        },
        logout: (state) => {
            state.user = null;
            storage.removeItem("token");
        },
    },
});

export const { loginSuccess, logout } = authSlice.actions;
export default authSlice.reducer;
