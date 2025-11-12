import {appDefault} from "$lib/api/api";
import type {PageLoad} from "./$types";


export const load: PageLoad = async () => {
    return {
        appDefault: await appDefault(),
    };
}

