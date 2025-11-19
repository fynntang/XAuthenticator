<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { addAccount } from "$lib/api/api";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { toast } from "svelte-sonner";

  export let open = false;

  const dispatch = createEventDispatcher();

  let authUrl = "";
  let loading = false;

  async function handleSubmit() {
    if (!authUrl.trim()) {
      toast.error("Please enter an OTP Auth URL");
      return;
    }

    loading = true;
    try {
      await addAccount(authUrl);
      toast.success("Account added successfully");
      authUrl = "";
      dispatch("success");
    } catch (error: any) {
      toast.error("Failed to add account", {
        description: error.reason || "Unknown error",
      });
    } finally {
      loading = false;
    }
  }

  function handleCancel() {
    authUrl = "";
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-[525px]">
    <Dialog.Header>
      <Dialog.Title>Add New Account</Dialog.Title>
      <Dialog.Description>
        Enter the OTP Auth URL from your authenticator app or scan a QR code.
      </Dialog.Description>
    </Dialog.Header>
    <form on:submit|preventDefault={handleSubmit}>
      <div class="grid gap-4 py-4">
        <div class="grid gap-2">
          <Label for="authUrl">OTP Auth URL</Label>
          <Input
            id="authUrl"
            bind:value={authUrl}
            placeholder="otpauth://totp/Example:user@example.com?secret=..."
            disabled={loading}
          />
          <p class="text-sm text-muted-foreground">
            The URL should start with "otpauth://"
          </p>
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
        <Button type="submit" disabled={loading || !authUrl.trim()}>
          {loading ? "Adding..." : "Add Account"}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
