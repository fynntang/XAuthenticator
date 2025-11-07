<script lang="ts">
    import {onMount} from 'svelte';
    import logo from '$lib/assets/logo.png';
    import {Progress} from "$lib/components/ui/progress";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {initialize} from "$lib/initialize";
    import {appState, launchApp} from "$lib/api/api";
    import {randomLaunchImage, wait} from "$lib/utils";
    import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
    import type {APIError} from "$lib/api/types";
    import {CommonError} from "$lib/api/errors";
    import {showWindow} from "$lib/window";

    let progress = $state(0);
    const initialized = initialize();
    const currentWindow = getCurrentWindow();

    type CopyrightKeys = 'date' | 'name';


    const runInitialization = async () => {
        try {
            progress = 10;
            let state = await appState();
            progress += 10;
            while (state && !state.isInitialized) {
                await launchApp()
                progress += 10;
                state = await appState();
                progress += 10;
                if (state && state.isInitialized) {
                    await initialized.createTray();
                    progress += 10;
                }
                await wait(1000)
            }
            progress = 90;
            await wait(300)
            await showWindow(WebviewWindowLabels.Main)
        } catch (e) {
            if ((e as APIError).code === CommonError.MasterKeyNotInitialized) {
                console.error(e);
                await showWindow(WebviewWindowLabels.Initialization);
            } else {
                alert("An error occurred during initialization: " + JSON.stringify(e));
            }
        } finally {
            progress = 90;
            await wait(300);
            progress = 100;
            await currentWindow.hide();
        }
    }


    onMount(() => {
        runInitialization()
    });
</script>
<div class="relative w-screen h-screen flex flex-col from-white to-gray-50 dark:from-neutral-900 dark:to-neutral-950">
    <div data-tauri-drag-region class="absolute top-0 left-0 w-full h-full z-10"></div>
    <div class="relative flex w-full h-full">
        <div class="relative flex flex-col w-3/5 p-6 pr-0">
            <div class="relative flex gap-4">
                <div class="flex-none size-9"
                     style:background-image="url({logo})"
                     style:background-size="cover"
                     style:background-position="center"
                ></div>
                <h1 class="flex text-3xl font-bold">XAuthenticator</h1>
            </div>
            <div class="flex mt-auto mb-4 text-gray-500 dark:text-gray-400">
                {__COPYRIGHT__.replace(/\{([^{}]+)\}/g, (match, placeholder: CopyrightKeys) => {
                    return {
                        date: (new Date()).getFullYear().toString(),
                        name: __NAME__,
                    }[placeholder] || match;
                })}
            </div>
        </div>
        <div class="flex w-2/5 h-full"
             style="background-image: url({randomLaunchImage()}); background-size: cover; background-position: center;">
        </div>
    </div>
    {#if progress < 100}
        <div class="flex flex-col absolute bottom-0 left-0 right-0">
            <div class="h-1 w-full overflow-hidden">
                <Progress value={progress}/>
            </div>
        </div>
    {/if}
</div>