<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { updateAccount } from "$lib/api/api";
  import type { Account } from "$lib/api/types";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";
  import { toast } from "svelte-sonner";

  export let open = false;
  export let account: Account;

  const dispatch = createEventDispatcher();

  let issuer = "";
  let label = "";
  let note = "";
  let loading = false;

  $: if (account && open) {
    issuer = account.issuer;
    label = account.label;
    note = "";
  }

  async function handleSubmit() {
    if (!issuer.trim() || !label.trim()) {
      toast.error("Issuer and label are required");
      return;
    }

    loading = true;
    try {
      await updateAccount({
        id: account.id,
        issuer: issuer !== account.issuer ? issuer : undefined,
        label: label !== account.label ? label : undefined,
        note: note || undefined,
      });
      toast.success("Account updated successfully");
      dispatch("success");
    } catch (error: any) {
      toast.error("Failed to update account", {
        description: error.reason || "Unknown error",
      });
    } finally {
      loading = false;
    }
  }

  function handleCancel() {
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-[525px]">
    <Dialog.Header>
      <Dialog.Title>Edit Account</Dialog.Title>
      <Dialog.Description>
        Update the account details. The secret and authentication settings cannot be changed.
      </Dialog.Description>
    </Dialog.Header>
    <form on:submit|preventDefault={handleSubmit}>
      <div class="grid gap-4 py-4">
        <div class="grid gap-2">
          <Label for="issuer">Issuer</Label>
          <Input
            id="issuer"
            bind:value={issuer}
            placeholder="Google"
            disabled={loading}
          />
        </div>
        <div class="grid gap-2">
          <Label for="label">Label</Label>
          <Input
            id="label"
            bind:value={label}
            placeholder="user@example.com"
            disabled={loading}
          />
        </div>
        <div class="grid gap-2">
          <Label for="note">Note (optional)</Label>
          <Textarea
            id="note"
            bind:value={note}
            placeholder="Add a note for this account..."
            disabled={loading}
          />
        </div>
        <div class="grid gap-2 opacity-50">
          <Label>Read-only Settings</Label>
          <div class="grid grid-cols-2 gap-2 text-sm">
            <div>
              <span class="text-muted-foreground">Type:</span>
              <span class="ml-2 font-medium">{account.type}</span>
            </div>
            <div>
              <span class="text-muted-foreground">Algorithm:</span>
              <span class="ml-2 font-medium">{account.algorithm}</span>
            </div>
            <div>
              <span class="text-muted-foreground">Digits:</span>
              <span class="ml-2 font-medium">{account.digits}</span>
            </div>
            {#if account.period}
              <div>
                <span class="text-muted-foreground">Period:</span>
                <span class="ml-2 font-medium">{account.period}s</span>
              </div>
            {/if}
          </div>
        </div>
      </div>
      <Dialog.Footer>
        <Button
          type="button"
          variant="outline"
          onclick={handleCancel}
          disabled={loading}
        >
          Cancel
        </Button>
        <Button type="submit" disabled={loading || (!issuer.trim() || !label.trim())}>
          {loading ? "Updating..." : "Update Account"}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
