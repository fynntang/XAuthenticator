// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
    namespace App {
        // interface Error {}
        // interface Locals {}
        // interface PageData {}
        // interface PageState {}
        // interface Platform {}
    }
    declare const __NAME__: string
    declare const __VERSION__: string
    declare const __REPOSITORY__:  string
    declare const __AUTHOR__: { name: string, email: string, url: string }
    declare const __LICENSE__: string
    declare const __COPYRIGHT__: string
}

export {};