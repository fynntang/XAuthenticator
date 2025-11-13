<script lang="ts">
    import {WebviewWindowLabels} from "$lib/constants/webview-window-labels";
    import Titlebar from "$lib/components/layout/titlebar.svelte";

    import {Button} from "$lib/components/ui/button";
    import {Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,} from "$lib/components/ui/card";
    import {InputGroup, InputGroupAddon, InputGroupInput,} from "$lib/components/ui/input-group";
    import {Eye, EyeOff, Lock} from "@lucide/svelte";

    import {unlockAppWithPassword} from "$lib/api/api";
    import {appStore} from "$lib/stores/stores";


    let password = $state("");
    let showPassword = $state(false);
    let error = $state("");
    let loading = $state(false);
    let {children} = $props();


    const onUnlock = async () => {
        if (!password || loading) return;
        error = "";
        loading = true;
        try {
            await unlockAppWithPassword(password);
        } catch (e: any) {
            error = e?.message ?? "解锁失败，请重试";
        } finally {
            loading = false;
        }
    };

    const onKeyDown = (e: KeyboardEvent) => {
        if (e.key === "Enter") onUnlock();
    };

</script>

{#if $appStore?.isLocked}
    <Titlebar windowLabels={WebviewWindowLabels.Main}/>
    <section class="grid place-items-center h-[calc(100vh-var(--header-height))] px-4">
        <Card class="w-full max-w-sm">
            <CardHeader>
                <CardTitle class="flex items-center gap-2">
                    <Lock class="text-muted-foreground"/>
                    应用已锁定
                </CardTitle>
                <CardDescription>请输入密码以解锁应用</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-3">
                <InputGroup aria-invalid={!!error}>
                    <InputGroupInput type={showPassword ? "text" : "password"} placeholder="密码"
                                     bind:value={password}
                                     onkeydown={onKeyDown} autocomplete="current-password" autofocus/>
                    <InputGroupAddon align="inline-end">
                        <Button variant="ghost" size="icon" onclick={() => (showPassword = !showPassword)}
                                aria-label={showPassword ? "隐藏密码" : "显示密码"}>
                            {#if showPassword}
                                <EyeOff/>
                            {:else}
                                <Eye/>
                            {/if}
                        </Button>
                    </InputGroupAddon>
                </InputGroup>
                {#if error}
                    <p class="text-destructive text-xs">{error}</p>
                {/if}
            </CardContent>
            <CardFooter class="flex gap-2">
                <Button class="flex-1" onclick={onUnlock} disabled={!password || loading}>
                    {#if loading}正在解锁...{:else}解锁{/if}
                </Button>
                <Button variant="outline" size="sm" onclick={() => (password = "")} disabled={loading}>清空</Button>
            </CardFooter>
        </Card>
    </section>
{:else}
    {@render children?.()}
{/if}
