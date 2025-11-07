import {type ClassValue, clsx} from "clsx";
import {twMerge} from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export const wait = (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms));


// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, "children"> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & { ref?: U | null };


const launchImages = ["1845852", "5742416", "6496937", "6834164", "7899206", "8258264", "9059825"];
export const randomLaunchImage = (): string => {
    const url = new URL(`./assets/launch/${launchImages[Math.floor(Math.random() * launchImages.length)]}.avif`, import.meta.url)
    console.log(url)
    return url.pathname
}
