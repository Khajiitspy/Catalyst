import {IImageFile} from "@/types/common/IImageFile";

export interface IProfileEdit {
    firstName: string;
    lastName: string;
    email: string;
    imageFile?: IImageFile;
}
