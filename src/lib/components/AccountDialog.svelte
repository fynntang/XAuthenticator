<script lang="ts">
    import type {Account} from "$lib/api/types";
    import {createEventDispatcher} from "svelte";
    import {Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter} from "$lib/components/ui/dialog";
    import {Button} from "$lib/components/ui/button";
    import {Input} from "$lib/components/ui/input";
    import {Label} from "$lib/components/ui/label";
    import {Textarea} from "$lib/components/ui/textarea";
    import {NativeSelect, NativeSelectOption} from "$lib/components/ui/native-select";
    
    export let open = false;
    export let account: Account | null = null;
    
    const dispatch = createEventDispatcher();
    
    let form = {
        name: '',
        issuer: '',
        accountName: '',
        secret: '',
        algorithm: 'SHA1',
        digits: 6,
        period: 30,
        notes: ''
    };
    
    $: if (open && account) {
        form = {
            name: account.name,
            issuer: account.issuer || '',
            accountName: account.accountName || '',
            secret: account.secret,
            algorithm: account.algorithm,
            digits: account.digits,
            period: account.period,
            notes: account.notes || ''
        };
    } else if (open && !account) {
        form = {
            name: '',
            issuer: '',
            accountName: '',
            secret: '',
            algorithm: 'SHA1',
            digits: 6,
            period: 30,
            notes: ''
        };
    }
    
    function handleSubmit() {
        const data: Partial<Account> = {
            name: form.name,
            secret: form.secret,
        };
        
        if (form.issuer) data.issuer = form.issuer;
        if (form.accountName) data.accountName = form.accountName;
        if (form.algorithm) data.algorithm = form.algorithm;
        if (form.digits) data.digits = form.digits;
        if (form.period) data.period = form.period;
        if (form.notes) data.notes = form.notes;
        
        dispatch('save', data);
    }
</script>

<Dialog bind:open>
    <DialogContent class="max-w-md">
        <DialogHeader>
            <DialogTitle>{account ? 'Edit Account' : 'Add Account'}</DialogTitle>
        </DialogHeader>
        
        <form on:submit|preventDefault={handleSubmit} class="space-y-4">
            <div>
                <Label for="name">Name *</Label>
                <Input
                    id="name"
                    bind:value={form.name}
                    placeholder="e.g., GitHub, Google"
                    required
                />
            </div>
            
            <div>
                <Label for="issuer">Issuer</Label>
                <Input
                    id="issuer"
                    bind:value={form.issuer}
                    placeholder="e.g., GitHub"
                />
            </div>
            
            <div>
                <Label for="accountName">Account Name</Label>
                <Input
                    id="accountName"
                    bind:value={form.accountName}
                    placeholder="e.g., user@example.com"
                />
            </div>
            
            <div>
                <Label for="secret">Secret Key *</Label>
                <Input
                    id="secret"
                    bind:value={form.secret}
                    placeholder="Base32 encoded secret"
                    required
                    type="password"
                />
                <p class="text-xs text-muted-foreground mt-1">
                    The secret key is usually provided as a QR code or text string
                </p>
            </div>
            
            <div class="grid grid-cols-3 gap-4">
                <div>
                    <Label for="algorithm">Algorithm</Label>
                    <NativeSelect bind:value={form.algorithm} id="algorithm">
                        <NativeSelectOption value="SHA1">SHA1</NativeSelectOption>
                        <NativeSelectOption value="SHA256">SHA256</NativeSelectOption>
                        <NativeSelectOption value="SHA512">SHA512</NativeSelectOption>
                    </NativeSelect>
                </div>
                
                <div>
                    <Label for="digits">Digits</Label>
                    <Input
                        id="digits"
                        type="number"
                        bind:value={form.digits}
                        min="6"
                        max="8"
                    />
                </div>
                
                <div>
                    <Label for="period">Period (s)</Label>
                    <Input
                        id="period"
                        type="number"
                        bind:value={form.period}
                        min="15"
                        max="60"
                    />
                </div>
            </div>
            
            <div>
                <Label for="notes">Notes</Label>
                <Textarea
                    id="notes"
                    bind:value={form.notes}
                    placeholder="Optional notes"
                    rows={3}
                />
            </div>
            
            <DialogFooter>
                <Button type="button" variant="outline" onclick={() => open = false}>
                    Cancel
                </Button>
                <Button type="submit">
                    {account ? 'Update' : 'Add'} Account
                </Button>
            </DialogFooter>
        </form>
    </DialogContent>
</Dialog>
