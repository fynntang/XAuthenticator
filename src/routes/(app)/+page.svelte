<script lang="ts">
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

    const sidebarWidth = Number(SIDEBAR_WIDTH.replace("rem", "")) * 16;

    const detail = {
        title: "My Google Account",
        username: "abc@google.com",
        createdAt: "2025-08-26",
        password: "••••••••••",
        otp: "297 • 324",
        otpTTL: 7,
        website: "https://accounts.google.com",
        tag: "My Google Account",
        lastEdited: "2025-08-26 11:34:54"
    };

</script>


<ResizablePaneGroup direction="horizontal">
    <ResizablePane minSize={25} defaultSize={30} maxSize={35} class="flex flex-col pl-4">
        <ScrollArea class="h-full w-full">
            <ItemGroup class="pr-4">
                {#each Array.from({length: 500}) as _,index(index)}
                    <Item>
                        {#snippet child({props})}
                            <a href="#/" {...props}>
                                <ItemMedia>
                                    <Avatar>
                                        <AvatarImage
                                                src={"https://ui-avatars.com/api/?name=XAuthenticator&format=svg&bold=true"}
                                                class="grayscale"/>
                                        <AvatarFallback>Name</AvatarFallback>
                                    </Avatar>
                                </ItemMedia>
                                <ItemContent class="gap-1">
                                    <ItemTitle>Name</ItemTitle>
                                    <ItemDescription>email</ItemDescription>
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
            <div class="flex flex-col gap-6 py-6 mx-auto w-full">
                <div class="[.border-b]:pb-6 grid auto-rows-min grid-cols-[auto_1fr_auto] items-center gap-4 px-6">
                    <Avatar class="size-12 rounded-lg">
                        <AvatarImage src="https://logo.clearbit.com/google.com"/>
                        <AvatarFallback>G</AvatarFallback>
                    </Avatar>
                    <div class="flex flex-col">
                        <CardTitle>{detail.title}</CardTitle>
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
                    <div class="rounded-xl border">
                        <div class="flex items-center justify-between gap-4 p-4">
                            <div class="flex flex-col">
                                <span class="text-sm text-muted-foreground">UserName</span>
                                <span class="text-base">{detail.username}</span>
                            </div>
                        </div>
                    </div>

                    <div class="rounded-xl border p-4">
                        <div class="flex items-center justify-between">
                            <div class="flex flex-col">
                                <span class="text-sm text-muted-foreground">Password</span>
                                <span class="tracking-widest">{detail.password}</span>
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

                    <div class="rounded-xl border p-4">
                        <div class="flex items-center justify-between">
                            <div class="flex flex-col">
                                <span class="text-sm text-muted-foreground">OTP</span>
                                <span class="text-base">{detail.otp}</span>
                            </div>
                            <Badge variant="outline" class="text-xs">{detail.otpTTL}</Badge>
                        </div>
                    </div>

                    <div class="rounded-xl border p-4">
                        <div class="flex items-center justify-between">
                            <div class="flex flex-col">
                                <span class="text-sm text-muted-foreground">Website</span>
                                <a href={detail.website}
                                   class="text-base underline underline-offset-4">{detail.website}</a>
                            </div>
                            <Globe class="size-4 opacity-70"/>
                        </div>
                    </div>

                    <div class="rounded-xl border p-4">
                        <span class="text-sm text-muted-foreground">Tags</span>
                        <div class="mt-2">
                            <Badge variant="secondary">{detail.tag}</Badge>
                        </div>
                    </div>

                    <Separator class="my-2 border-transparent"/>
                    <div class="text-muted-foreground text-sm">Latest Update {detail.lastEdited}</div>
                </div>

            </div>
        </ScrollArea>
    </ResizablePane>
</ResizablePaneGroup>