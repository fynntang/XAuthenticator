<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Textarea } from "$lib/components/ui/textarea";
    import * as Select from "$lib/components/ui/select";
    import type { CreateAccountRequest, UpdateAccountRequest, Account } from "$lib/api/types";
    import { addAccount, updateAccount } from "$lib/api/api";
    
    interface Props {
        open: boolean;
        mode: "add" | "edit";
        account?: Account;
        password: string;
        onOpenChange?: (open: boolean) => void;
        onSuccess?: () => void;
    }
    
    let { open = $bindable(), mode, account, password, onOpenChange, onSuccess }: Props = $props();
    
    let formData = $state({
        issuer: account?.issuer || "",
        label: account?.label || "",
        type: account?.type || "TOTP",
        algorithm: account?.algorithm || "SHA1",
        digits: account?.digits || 6,
        period: account?.period || 30,
        counter: account?.counter || 0,
        secret: "",
        note: account?.note || "",
    });
    
    let loading = $state(false);
    let error = $state<string | null>(null);
    
    async function handleSubmit() {
        if (!password) {
            error = "Password is required";
            return;
        }
        
        if (!formData.issuer.trim()) {
            error = "Issuer is required";
            return;
        }
        
        if (mode === "add" && !formData.secret.trim()) {
            error = "Secret is required";
            return;
        }
        
        loading = true;
        error = null;
        
        try {
            if (mode === "add") {
                const request: CreateAccountRequest = {
                    issuer: formData.issuer,
                    label: formData.label,
                    type: formData.type,
                    algorithm: formData.algorithm,
                    digits: formData.digits,
                    period: formData.type === "TOTP" ? formData.period : undefined,
                    counter: formData.type === "HOTP" ? formData.counter : undefined,
                    secret: formData.secret,
                    note: formData.note || undefined,
                };
                await addAccount(request, password);
            } else if (mode === "edit" && account) {
                const request: UpdateAccountRequest = {
                    id: account.id,
                    issuer: formData.issuer !== account.issuer ? formData.issuer : undefined,
                    label: formData.label !== account.label ? formData.label : undefined,
                    note: formData.note !== account.note ? formData.note : undefined,
                };
                await updateAccount(request, password);
            }
            
            open = false;
            onSuccess?.();
        } catch (err: any) {
            console.error("Failed to save account:", err);
            error = err.reason || "Failed to save account";
        } finally {
            loading = false;
        }
    }
    
    function handleOpenChange(newOpen: boolean) {
        open = newOpen;
        onOpenChange?.(newOpen);
        
        if (!newOpen) {
            // Reset form on close
            error = null;
            if (mode === "add") {
                formData = {
                    issuer: "",
                    label: "",
                    type: "TOTP",
                    algorithm: "SHA1",
                    digits: 6,
                    period: 30,
                    counter: 0,
                    secret: "",
                    note: "",
                };
            }
        }
    }
</script>

<Dialog.Root {open} onOpenChange={handleOpenChange}>
    <Dialog.Content class="sm:max-w-[500px]">
        <Dialog.Header>
            <Dialog.Title>
                {mode === "add" ? "Add Account" : "Edit Account"}
            </Dialog.Title>
            <Dialog.Description>
                {mode === "add" 
                    ? "Add a new 2FA account to your database" 
                    : "Update account information"}
            </Dialog.Description>
        </Dialog.Header>
        
        <div class="grid gap-4 py-4">
            {#if error}
                <div class="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
                    {error}
                </div>
            {/if}
            
            <div class="grid gap-2">
                <Label for="issuer">Issuer *</Label>
                <Input 
                    id="issuer" 
                    bind:value={formData.issuer}
                    placeholder="e.g., Google, GitHub"
                    disabled={loading}
                />
            </div>
            
            <div class="grid gap-2">
                <Label for="label">Account Label</Label>
                <Input 
                    id="label" 
                    bind:value={formData.label}
                    placeholder="e.g., user@example.com"
                    disabled={loading}
                />
            </div>
            
            {#if mode === "add"}
                <div class="grid gap-2">
                    <Label for="secret">Secret *</Label>
                    <Input 
                        id="secret" 
                        type="password"
                        bind:value={formData.secret}
                        placeholder="Base32 encoded secret"
                        disabled={loading}
                    />
                </div>
                
                <div class="grid grid-cols-2 gap-4">
                    <div class="grid gap-2">
                        <Label for="type">Type</Label>
                        <Select.Root 
                            selected={{ value: formData.type, label: formData.type }}
                            onSelectedChange={(v) => formData.type = v?.value || "TOTP"}
                        >
                            <Select.Trigger id="type" disabled={loading}>
                                <Select.Value placeholder="Select type" />
                            </Select.Trigger>
                            <Select.Content>
                                <Select.Item value="TOTP">TOTP</Select.Item>
                                <Select.Item value="HOTP">HOTP</Select.Item>
                            </Select.Content>
                        </Select.Root>
                    </div>
                    
                    <div class="grid gap-2">
                        <Label for="algorithm">Algorithm</Label>
                        <Select.Root 
                            selected={{ value: formData.algorithm, label: formData.algorithm }}
                            onSelectedChange={(v) => formData.algorithm = v?.value || "SHA1"}
                        >
                            <Select.Trigger id="algorithm" disabled={loading}>
                                <Select.Value placeholder="Select algorithm" />
                            </Select.Trigger>
                            <Select.Content>
                                <Select.Item value="SHA1">SHA1</Select.Item>
                                <Select.Item value="SHA256">SHA256</Select.Item>
                                <Select.Item value="SHA512">SHA512</Select.Item>
                            </Select.Content>
                        </Select.Root>
                    </div>
                </div>
                
                <div class="grid grid-cols-2 gap-4">
                    <div class="grid gap-2">
                        <Label for="digits">Digits</Label>
                        <Input 
                            id="digits" 
                            type="number"
                            bind:value={formData.digits}
                            min="6"
                            max="8"
                            disabled={loading}
                        />
                    </div>
                    
                    {#if formData.type === "TOTP"}
                        <div class="grid gap-2">
                            <Label for="period">Period (seconds)</Label>
                            <Input 
                                id="period" 
                                type="number"
                                bind:value={formData.period}
                                min="15"
                                max="60"
                                disabled={loading}
                            />
                        </div>
                    {:else}
                        <div class="grid gap-2">
                            <Label for="counter">Counter</Label>
                            <Input 
                                id="counter" 
                                type="number"
                                bind:value={formData.counter}
                                min="0"
                                disabled={loading}
                            />
                        </div>
                    {/if}
                </div>
            {/if}
            
            <div class="grid gap-2">
                <Label for="note">Note</Label>
                <Textarea 
                    id="note" 
                    bind:value={formData.note}
                    placeholder="Optional note"
                    rows={3}
                    disabled={loading}
                />
            </div>
        </div>
        
        <Dialog.Footer>
            <Button variant="outline" onclick={() => handleOpenChange(false)} disabled={loading}>
                Cancel
            </Button>
            <Button onclick={handleSubmit} disabled={loading}>
                {loading ? "Saving..." : "Save"}
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
