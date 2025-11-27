<script lang="ts">
    import type {PageProps} from './$types';

    import {SIDEBAR_WIDTH} from "$lib/components/ui/sidebar/constants";
    import {ResizableHandle, ResizablePane, ResizablePaneGroup} from "$lib/components/ui/resizable";
    import {Item, ItemContent, ItemDescription, ItemGroup, ItemMedia, ItemTitle} from "$lib/components/ui/item";
    import {Avatar, AvatarFallback, AvatarImage} from "$lib/components/ui/avatar";
    import {ScrollArea} from "$lib/components/ui/scroll-area";
    import {CardTitle} from "$lib/components/ui/card";
    import {Button} from "$lib/components/ui/button";
    import {Badge} from "$lib/components/ui/badge";
    import {Separator} from "$lib/components/ui/separator";
    import {Copy, Eye, Globe, Pencil, Share} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {listAccounts} from "$lib/api/api";
    import type {Entry} from "$lib/api/types";

    const sidebarWidth = Number(SIDEBAR_WIDTH.replace("rem", "")) * 16;
    let {data}: PageProps = $props();
    let accountsState: Entry[] = $state<Entry[]>([]);
    let currentAccountState = $state<Entry | null>(null);

    onMount(async () => {
        accountsState = await listAccounts()
    })

    $inspect("data", data, accountsState)
</script>


<ResizablePaneGroup direction="horizontal">
    <ResizablePane minSize={25} defaultSize={30} maxSize={35} class="flex flex-col pl-4">
        <ScrollArea class="h-full w-full">
            <ItemGroup class="pr-4">
                {#each accountsState as account,index(index)}
                    <Item>
                        {#snippet child({props})}
                            <a {...props} onclick={()=>{ currentAccountState = account; }}>
                                <ItemMedia>
                                    <Avatar>
                                        {#if account.fields.website}
                                            <AvatarImage class="grayscale"
                                                         src={`https://logo.5io.cc/${URL.parse(account.fields.website)?.host}`}/>
                                        {:else}
                                            <AvatarImage class="grayscale"
                                                         src={`https://ui-avatars.com/api/?name=${account.fields.UserName}&format=svg&bold=true&background=random&rounded=true`}/>
                                        {/if}
                                        <AvatarFallback>{account.fields.UserName}</AvatarFallback>
                                    </Avatar>
                                </ItemMedia>
                                <ItemContent class="gap-1">
                                    <ItemTitle>{account.fields.Title}</ItemTitle>
                                    <ItemDescription>{account.fields.UserName}</ItemDescription>
                                </ItemContent>
                            </a>
                        {/snippet}
                    </Item>
                {/each}
            </ItemGroup>
        </ScrollArea>
    </ResizablePane>
    <ResizableHandle withHandle/>
    <ResizablePane minSize={65} defaultSize={70} maxSize={75} class="flex flex-col px-4">
        <ScrollArea class="h-full w-full">
            {#if currentAccountState}
                <div class="flex flex-col gap-6 py-6 mx-auto w-full">
                    <div class="[.border-b]:pb-6 grid auto-rows-min grid-cols-[auto_1fr_auto] items-center gap-4 px-6">
                        <Avatar class="size-12 rounded-lg">
                            {#if currentAccountState.fields.website}
                                <AvatarImage src={`https://logo.5io.cc/${URL.parse(currentAccountState.fields.website)?.host}`}/>
                            {:else}
                                <AvatarImage class="grayscale"
                                             src={`https://ui-avatars.com/api/?name=${currentAccountState.fields.UserName}&format=svg&bold=true&background=random&rounded=true`}/>
                            {/if}
                            <AvatarFallback>{currentAccountState?.fields.UserName}</AvatarFallback>
                        </Avatar>
                        <div class="flex flex-col">
                            <CardTitle>{currentAccountState?.fields.Title}</CardTitle>
                        </div>
                        <div class="flex items-center gap-2">
                            <Button variant="ghost" size="icon">
                                <Share class="size-4"/>
                            </Button>
                            <Button variant="ghost" size="icon">
                                <Pencil class="size-4"/>
                            </Button>
                        </div>
                    </div>
                    <div class="flex flex-col px-6 gap-4">

                        {#if currentAccountState?.fields.UserName}
                            <div class="rounded-xl border">
                                <div class="flex items-center justify-between gap-4 p-4">
                                    <div class="flex flex-col">
                                        <span class="text-sm text-muted-foreground">UserName</span>
                                        <span class="text-base">{currentAccountState?.fields.UserName}</span>
                                    </div>
                                </div>
                            </div>
                        {/if}

                        {#if currentAccountState?.fields.Password}
                            <div class="rounded-xl border p-4">
                                <div class="flex items-center justify-between">
                                    <div class="flex flex-col">
                                        <span class="text-sm text-muted-foreground">Password</span>
                                        <span class="tracking-widest">{currentAccountState?.fields.Password}</span>
                                    </div>
                                    <div class="flex items-center gap-3">
                                        <div class="flex items-center gap-1 text-xs">
                                            <span class="size-2 rounded-full bg-emerald-500"></span>
                                            <span>极佳</span>
                                        </div>
                                        <Button variant="ghost" size="icon">
                                            <Copy class="size-4"/>
                                        </Button>
                                        <Button variant="ghost" size="icon">
                                            <Eye class="size-4"/>
                                        </Button>
                                    </div>
                                </div>
                            </div>
                        {/if}

                        {#if currentAccountState?.fields.otp}
                            <div class="rounded-xl border p-4">
                                <div class="flex items-center justify-between">
                                    <div class="flex flex-col">
                                        <span class="text-sm text-muted-foreground">OTP</span>
                                        <span class="text-base">{currentAccountState?.fields.otp}</span>
                                    </div>
                                    <Badge variant="outline"
                                           class="text-xs">{currentAccountState?.fields.otpTTL}</Badge>
                                </div>
                            </div>
                        {/if}

                        {#if currentAccountState?.fields.website}
                            <div class="rounded-xl border p-4">
                                <div class="flex items-center justify-between">
                                    <div class="flex flex-col">
                                        <span class="text-sm text-muted-foreground">Website</span>
                                        <a href={currentAccountState?.fields.website}
                                           class="text-base underline underline-offset-4">{currentAccountState?.fields.website}</a>
                                    </div>
                                    <Globe class="size-4 opacity-70"/>
                                </div>
                            </div>
                        {/if}

                        {#if currentAccountState?.fields.tag}
                            <div class="rounded-xl border p-4">
                                <span class="text-sm text-muted-foreground">Tags</span>
                                <div class="mt-2">
                                    <Badge variant="secondary">{currentAccountState?.fields.tag}</Badge>
                                </div>
                            </div>
                        {/if}
                        <Separator class="my-2 border-transparent"/>
                        <div class="text-muted-foreground text-sm">Latest
                            Update {currentAccountState?.fields.lastEdited}</div>
                    </div>

                </div>
            {/if}
        </ScrollArea>
    </ResizablePane>
</ResizablePaneGroup>