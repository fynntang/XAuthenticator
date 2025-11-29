<script lang="ts">
    import {Button} from "$lib/components/ui/button";
    import {Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,} from "$lib/components/ui/card";
    import {InputGroup, InputGroupAddon, InputGroupInput,} from "$lib/components/ui/input-group";
    import {Eye, EyeOff, Lock} from "@lucide/svelte";

    import {appState, unlockAppWithPassword} from "$lib/api/api";
    import {appStateChange, appStore} from "$lib/stores/stores";
    import {_ as t} from 'svelte-i18n';
    import {Spinner} from "$lib/components/ui/spinner";


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
            appStateChange(await appState());
        } catch (e: any) {
            error = e?.message ?? $t('appLock.unlockFailed');
        } finally {
            loading = false;
        }
    };

    const onKeyDown = (e: KeyboardEvent) => {
        if (e.key === "Enter") onUnlock();
    };

</script>

{#if $appStore?.isLocked}
    <section class="grid place-items-center h-[calc(100vh-var(--header-height))] px-4">
        <Card class="w-full max-w-sm">
            <CardHeader>
                <CardTitle class="flex items-center gap-2">
                    <Lock class="text-muted-foreground"/>
                    {$t('appLock.title')}
                </CardTitle>
                <CardDescription>{$t('appLock.description')}</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-3">
                <InputGroup aria-invalid={!!error}>
                    <InputGroupInput type={showPassword ? "text" : "password"}
                                     placeholder={$t('appLock.passwordPlaceholder')}
                                     bind:value={password}
                                     onkeydown={onKeyDown} autocomplete="current-password" autofocus/>
                    <InputGroupAddon align="inline-end">
                        <Button variant="ghost" size="icon" onclick={() => (showPassword = !showPassword)}
                                aria-label={showPassword ? $t('appLock.hidePassword') : $t('appLock.showPassword')}>
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
                    {#if loading}
                        <Spinner/>
                        {$t('appLock.unlocking')}
                    {:else}
                        {$t('appLock.unlock')}
                    {/if}
                </Button>
                <Button variant="outline" size="sm" onclick={() => (password = "")}
                        disabled={loading}>{$t('appLock.clear')}</Button>
            </CardFooter>
        </Card>
    </section>
{:else}
    {@render children?.()}
{/if}
