<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import {
        Dialog,
        DialogContent,
        DialogDescription,
        DialogFooter,
        DialogHeader,
        DialogTitle,
    } from "$lib/components/ui/dialog";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Textarea } from "$lib/components/ui/textarea";
    import {
        accountDialogState,
        refreshAccountsTrigger,
    } from "$lib/stores/stores";
    import { createAccount, updateAccount } from "$lib/api/api";
    import { toast } from "svelte-sonner";

    let open = $state(false);
    let mode = $state<"create" | "edit">("create");
    let accountId = $state("");
    let title = $state("");
    let username = $state("");
    let password = $state("");
    let url = $state("");
    let notes = $state("");
    let totp = $state("");

    accountDialogState.subscribe((state) => {
        open = state.open;
        mode = state.mode;
        if (state.account) {
            accountId = state.account.id;
            title = state.account.title;
            username = state.account.username;
            password = state.account.password;
            url = state.account.url;
            notes = state.account.notes;
            totp = state.account.totp || "";
        } else {
            accountId = "";
            title = "";
            username = "";
            password = "";
            url = "";
            notes = "";
            totp = "";
        }
    });

    const onOpenChange = (v: boolean) => {
        accountDialogState.update((s) => ({ ...s, open: v }));
    };

    const handleSubmit = async () => {
        try {
            if (mode === "create") {
                await createAccount({
                    title,
                    username,
                    password,
                    url,
                    notes,
                    totp,
                });
                toast.success("Account created successfully");
            } else {
                await updateAccount({
                    id: accountId,
                    title,
                    username,
                    password,
                    url,
                    notes,
                    totp,
                });
                toast.success("Account updated successfully");
            }
            accountDialogState.update((s) => ({ ...s, open: false }));
            refreshAccountsTrigger.update((n) => n + 1);
        } catch (e) {
            console.error(e);
            toast.error("Failed to save account");
        }
    };
</script>

<Dialog bind:open {onOpenChange}>
    <DialogContent class="sm:max-w-[425px]">
        <DialogHeader>
            <DialogTitle
                >{mode === "create"
                    ? "Create Account"
                    : "Edit Account"}</DialogTitle
            >
            <DialogDescription>
                {mode === "create"
                    ? "Add a new account to your vault."
                    : "Make changes to your account here."}
            </DialogDescription>
        </DialogHeader>
        <div class="grid gap-4 py-4">
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="title" class="text-right">Title</Label>
                <Input id="title" bind:value={title} class="col-span-3" />
            </div>
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="username" class="text-right">Username</Label>
                <Input id="username" bind:value={username} class="col-span-3" />
            </div>
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="password" class="text-right">Password</Label>
                <Input
                    id="password"
                    type="password"
                    bind:value={password}
                    class="col-span-3"
                />
            </div>
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="url" class="text-right">URL</Label>
                <Input id="url" bind:value={url} class="col-span-3" />
            </div>
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="totp" class="text-right">TOTP Secret</Label>
                <Input
                    id="totp"
                    bind:value={totp}
                    class="col-span-3"
                    placeholder="Optional"
                />
            </div>
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="notes" class="text-right">Notes</Label>
                <Textarea id="notes" bind:value={notes} class="col-span-3" />
            </div>
        </div>
        <DialogFooter>
            <Button type="submit" onclick={handleSubmit}>Save changes</Button>
        </DialogFooter>
    </DialogContent>
</Dialog>
