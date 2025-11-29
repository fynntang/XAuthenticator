<script lang="ts">
    import {
        ResizableHandle,
        ResizablePane,
        ResizablePaneGroup,
    } from "$lib/components/ui/resizable";
    import {
        Item,
        ItemContent,
        ItemDescription,
        ItemGroup,
        ItemMedia,
        ItemTitle,
    } from "$lib/components/ui/item";
    import {
        Avatar,
        AvatarFallback,
        AvatarImage,
    } from "$lib/components/ui/avatar";
    import { ScrollArea } from "$lib/components/ui/scroll-area";
    import { CardTitle } from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    import { Copy, Eye, Globe, Pencil, Share } from "@lucide/svelte";
    import { onMount, onDestroy } from "svelte";
    import { listAccounts } from "$lib/api/api";
    import type { Account } from "$lib/api/types";
    import AccountDialog from "$lib/components/account-dialog.svelte";
    import {
        accountDialogState,
        refreshAccountsTrigger,
    } from "$lib/stores/stores";

    let accountsState: Account[] = $state<Account[]>([]);
    let currentAccountState = $state<Account | null>(null);
    let unsubscribe: (() => void) | undefined;

    const getUrlHost = (url: string): string | null => {
        try {
            return new URL(url).host;
        } catch {
            return null;
        }
    };

    onMount(async () => {
        accountsState = await listAccounts();
        const storedId = sessionStorage.getItem("currentAccountId");
        if (storedId) {
            currentAccountState = accountsState.find(a => a.id === storedId) ?? null;
        }

        unsubscribe = refreshAccountsTrigger.subscribe(async () => {
            accountsState = await listAccounts();
        });
    });

    onDestroy(() => {
        unsubscribe?.();
    });

    const changeCurrentAccountState = (currentAccount: Account) => {
        currentAccountState = currentAccount;
        sessionStorage.setItem("currentAccountId", currentAccount.id);
    };
</script>

<ResizablePaneGroup direction="horizontal">
    <ResizablePane
        minSize={25}
        defaultSize={30}
        maxSize={35}
        class="flex flex-col pl-4"
    >
        <ScrollArea class="h-full w-full">
            <ItemGroup class="pr-4">
                {#each accountsState as account, index (index)}
                    <Item>
                        {#snippet child({ props })}
                            <a
                                {...props}
                                onclick={() => {
                                    changeCurrentAccountState(account);
                                }}
                            >
                                <ItemMedia>
                                    <Avatar>
                                        {#if account.url && getUrlHost(account.url)}
                                            <AvatarImage
                                                class="grayscale"
                                                src={`https://logo.5io.cc/${getUrlHost(account.url)}`}
                                            />
                                        {:else}
                                            <AvatarImage
                                                class="grayscale"
                                                src={`https://ui-avatars.com/api/?name=${account.username}&format=svg&bold=true&background=random&rounded=true`}
                                            />
                                        {/if}
                                        <AvatarFallback
                                            >{account.username}</AvatarFallback
                                        >
                                    </Avatar>
                                </ItemMedia>
                                <ItemContent class="gap-1">
                                    <ItemTitle>{account.title}</ItemTitle>
                                    <ItemDescription
                                        >{account.username}</ItemDescription
                                    >
                                </ItemContent>
                            </a>
                        {/snippet}
                    </Item>
                {/each}
            </ItemGroup>
        </ScrollArea>
    </ResizablePane>
    <ResizableHandle withHandle />
    <ResizablePane
        minSize={65}
        defaultSize={70}
        maxSize={75}
        class="flex flex-col px-4"
    >
        <ScrollArea class="h-full w-full">
            {#if currentAccountState}
                <div class="flex flex-col gap-6 py-6 mx-auto w-full">
                    <div
                        class="[.border-b]:pb-6 grid auto-rows-min grid-cols-[auto_1fr_auto] items-center gap-4 px-6"
                    >
                        <Avatar class="size-12 rounded-lg">
                            {#if currentAccountState.url && getUrlHost(currentAccountState.url)}
                                <AvatarImage
                                    src={`https://logo.5io.cc/${getUrlHost(currentAccountState.url)}`}
                                />
                            {:else}
                                <AvatarImage
                                    class="grayscale"
                                    src={`https://ui-avatars.com/api/?name=${currentAccountState.username}&format=svg&bold=true&background=random&rounded=true`}
                                />
                            {/if}
                            <AvatarFallback
                                >{currentAccountState?.username}</AvatarFallback
                            >
                        </Avatar>
                        <div class="flex flex-col">
                            <CardTitle>{currentAccountState?.title}</CardTitle>
                        </div>
                        <div class="flex items-center gap-2">
                            <Button variant="ghost" size="icon">
                                <Share class="size-4" />
                            </Button>
                            <Button
                                variant="ghost"
                                size="icon"
                                onclick={() => {
                                    if (currentAccountState) {
                                        accountDialogState.set({
                                            open: true,
                                            mode: "edit",
                                            account: currentAccountState,
                                        });
                                    }
                                }}
                            >
                                <Pencil class="size-4" />
                            </Button>
                        </div>
                    </div>
                    <div class="flex flex-col px-6 gap-4">
                        {#if currentAccountState?.username}
                            <div class="rounded-xl border">
                                <div
                                    class="flex items-center justify-between gap-4 p-4"
                                >
                                    <div class="flex flex-col">
                                        <span
                                            class="text-sm text-muted-foreground"
                                            >UserName</span
                                        >
                                        <span class="text-base"
                                            >{currentAccountState?.username}</span
                                        >
                                    </div>
                                </div>
                            </div>
                        {/if}

                        {#if currentAccountState?.password}
                            <div class="rounded-xl border p-4">
                                <div class="flex items-center justify-between">
                                    <div class="flex flex-col">
                                        <span
                                            class="text-sm text-muted-foreground"
                                            >Password</span
                                        >
                                        <span class="tracking-widest"
                                            >{currentAccountState?.password}</span
                                        >
                                    </div>
                                    <div class="flex items-center gap-3">
                                        <div
                                            class="flex items-center gap-1 text-xs"
                                        >
                                            <span
                                                class="size-2 rounded-full bg-emerald-500"
                                            ></span>
                                            <span>极佳</span>
                                        </div>
                                        <Button variant="ghost" size="icon">
                                            <Copy class="size-4" />
                                        </Button>
                                        <Button variant="ghost" size="icon">
                                            <Eye class="size-4" />
                                        </Button>
                                    </div>
                                </div>
                            </div>
                        {/if}

                        {#if currentAccountState?.totp}
                            <div class="rounded-xl border p-4">
                                <div class="flex items-center justify-between">
                                    <div class="flex flex-col">
                                        <span
                                            class="text-sm text-muted-foreground"
                                            >OTP</span
                                        >
                                        <span class="text-base"
                                            >{currentAccountState?.totp}</span
                                        >
                                    </div>
                                </div>
                            </div>
                        {/if}

                        {#if currentAccountState?.url}
                            <div class="rounded-xl border p-4">
                                <div class="flex items-center justify-between">
                                    <div class="flex flex-col">
                                        <span
                                            class="text-sm text-muted-foreground"
                                            >Website</span
                                        >
                                        <a
                                            href={currentAccountState?.url}
                                            class="text-base underline underline-offset-4"
                                            >{currentAccountState?.url}</a
                                        >
                                    </div>
                                    <Globe class="size-4 opacity-70" />
                                </div>
                            </div>
                        {/if}

                        {#if currentAccountState?.notes}
                            <div class="rounded-xl border p-4">
                                <span class="text-sm text-muted-foreground"
                                    >Notes</span
                                >
                                <div class="mt-2">
                                    <span class="text-base"
                                        >{currentAccountState?.notes}</span
                                    >
                                </div>
                            </div>
                        {/if}
                        <Separator class="my-2 border-transparent" />
                    </div>
                </div>
            {/if}
        </ScrollArea>
    </ResizablePane>
</ResizablePaneGroup>
<AccountDialog />
