<script lang="ts">
    import {onMount} from 'svelte';
    import {browser} from '$app/environment';
    import logo from '$lib/assets/logo.png';
    import {Progress} from "$lib/components/ui/progress";
    import img1845852 from "$lib/assets/launch/1845852.avif";
    import img5742416 from "$lib/assets/launch/5742416.avif";
    import img6496937 from "$lib/assets/launch/6496937.avif";
    import img6834164 from "$lib/assets/launch/6834164.avif";
    import img7899206 from "$lib/assets/launch/7899206.avif";
    import img8258264 from "$lib/assets/launch/8258264.avif";
    import img9059825 from "$lib/assets/launch/9059825.avif";

    let launchImages = [img1845852, img5742416, img6496937, img6834164, img7899206, img8258264, img9059825];
    let progress = $state(0);

    const wait = (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms));


    async function runInitialization() {
        try {
            progress = 0;
            await wait(300);
            const steps = [
                {msg: '检查运行环境', ms: 300},
                {msg: '加载应用配置', ms: 400},
                {msg: '初始化本地存储', ms: 350},
                {msg: '预热数据库连接', ms: 450},
                {msg: '载入用户偏好', ms: 300}
            ];

            for (let i = 0; i < steps.length; i++) {
                const step = steps[i];
                await wait(step.ms);
                progress = Math.round(((i + 1) / steps.length) * 100);
            }

            await wait(300);
        } catch (e) {
            console.error(e);
        }
    }


    onMount(() => {
        if (browser) {
            runInitialization();
        }
    });
</script>
<div class="relative w-screen h-screen flex flex-col from-white to-gray-50 dark:from-neutral-900 dark:to-neutral-950">
    <div class="relative flex w-full h-full">
        <div class="relative flex flex-col w-3/5 p-6 pr-0">
            <div class="relative flex">
                <div class="flex-none w-9 h-9 rounded-lg mr-4"
                     style:background-image="url({logo})"
                     style:background-size="cover"
                     style:background-position="center"
                ></div>
                <h1 class="flex text-3xl font-bold">XAuthenticator</h1>
            </div>
            <div class="flex mt-auto mb-4 text-gray-500 dark:text-gray-400">
                Copyright © {(new Date()).getFullYear()} XAuthenticator Contributors
            </div>
        </div>
        <div class="flex w-2/5 h-full"
             style="background-image: url({launchImages[Math.floor(Math.random()*7)]}); background-size: cover; background-position: center;">
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