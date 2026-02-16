import { useEffect } from "react";
import storage from "@/store/storages/tokenStorage";
import { useDispatch } from "react-redux";
import { loginSuccess } from "@/store/slices/authSlice";

export const useRestoreAuth = () => {
    const dispatch = useDispatch();

    useEffect(() => {
        const restore = async () => {
            const token = await storage.getItem("token");

            if (token) {
                dispatch(loginSuccess(token));
            }
        };

        restore();
    }, []);
};
