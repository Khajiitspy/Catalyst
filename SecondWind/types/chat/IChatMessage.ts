export interface IChatMessage {
    id: number;
    chatId: number;
    userId: number;
    message: string;
    createdAt: string;
    isMine: boolean;
    currentUserId: number;
}
