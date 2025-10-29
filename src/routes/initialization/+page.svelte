<script lang="ts">
    import type {PageProps} from './$types';

    import {Button} from "$lib/components/ui/button";
    import {Card, CardContent, CardFooter, CardHeader, CardTitle} from "$lib/components/ui/card";
    import {InputGroup, InputGroupAddon, InputGroupInput} from "$lib/components/ui/input-group";
    import {Alert, AlertDescription, AlertTitle} from "$lib/components/ui/alert";
    import {Progress} from "$lib/components/ui/progress";
    import {AlertCircle, Eye, EyeOff, Lock} from "@lucide/svelte";
    import {Spinner} from "$lib/components/ui/spinner";
    import img1845852 from "$lib/assets/launch/1845852.avif";
    import img5742416 from "$lib/assets/launch/5742416.avif";
    import img6496937 from "$lib/assets/launch/6496937.avif";
    import img6834164 from "$lib/assets/launch/6834164.avif";
    import img7899206 from "$lib/assets/launch/7899206.avif";
    import img8258264 from "$lib/assets/launch/8258264.avif";
    import img9059825 from "$lib/assets/launch/9059825.avif";
    import {Label} from "$lib/components/ui/label";
    import {Checkbox} from "$lib/components/ui/checkbox";

    let {params, data}: PageProps = $props();
    let password = $state("");
    let confirmPassword = $state("");
    let showPassword = $state(false);
    let showConfirm = $state(false);
    let acknowledged = $state(false);
    let error = $state("");
    let loading = $state(false);

    type Strength = { score: number; label: string; color: string; hints: string[] };
    let strength: Strength = $state({score: 0, label: "frail", color: "red", hints: []});
    let launchImages = [img1845852, img5742416, img6496937, img6834164, img7899206, img8258264, img9059825];


    const hasUpper = (s: string) => /[A-Z]/.test(s);
    const hasLower = (s: string) => /[a-z]/.test(s);
    const hasDigit = (s: string) => /\d/.test(s);
    const hasSymbol = (s: string) => /[^A-Za-z0-9]/.test(s);

    const evaluateStrength = (pw: string): Strength => {
        const hints: string[] = [];
        let score = 0;
        if (pw.length >= 12) score += 1; else hints.push("At least 12 characters");
        if (hasUpper(pw) && hasLower(pw)) score += 1; else hints.push("Contains uppercase and lowercase letters");
        if (hasDigit(pw)) score += 1; else hints.push("Contains numbers");
        if (hasSymbol(pw)) score += 1; else hints.push("Include symbols (e.g. !@#%)");
        const labels = ["frail", "general", "Medium", "good", "strong"];
        const colors = ["red", "orange", "yellow", "emerald", "green"];
        const idx = Math.min(score, 4);
        return {score, label: labels[idx], color: colors[idx], hints};
    };

    $effect(() => {
        strength = evaluateStrength(password);
        error = "";
    });

    const validate = (): boolean => {
        if (!password || !confirmPassword) {
            error = "Please enter your master password and confirm it again";
            return false;
        }
        if (password !== confirmPassword) {
            error = "The password entered twice is inconsistent";
            return false;
        }
        if (strength.score < 3) {
            error = "If the password is too low, increase the complexity";
            return false;
        }
        if (!acknowledged) {
            error = "Make sure you have saved your master password";
            return false;
        }
        return true;
    };

    const onInitialize = async () => {
        error = "";
        if (!validate()) return;
        loading = true;

        console.log("Start initializing");
    };


    $inspect(params, data, loading)
</script>


<main data-tauri-drag-region class="relative flex min-h-screen w-screen items-center justify-center p-4"
      style:background="url({launchImages[Math.floor(Math.random()*launchImages.length)]}) center/cover no-repeat">
    <Card data-tauri-drag-region class="relative w-full max-w-2xl z-[2]">
        <CardHeader>
            <CardTitle class="flex items-center gap-2">
                <Lock class="text-muted-foreground"/>
                The {__NAME__} needs to be initialized
            </CardTitle>
        </CardHeader>
        <CardContent class="grid gap-4">
            <div class="grid gap-3">
                <Label class="text-sm font-medium">Password</Label>
                <InputGroup aria-invalid={!!error}>
                    <InputGroupInput type={showPassword ? "text" : "password"} bind:value={password}
                                     autocomplete="new-password" autocapitalize="off" spellcheck={false}
                                     placeholder="Please enter your master password (12 digits or more recommended)"/>
                    <InputGroupAddon align="inline-end">
                        <Button variant="ghost" size="icon" onclick={() => (showPassword = !showPassword)}
                                aria-label={showPassword ? "Hide password" : "Show password"}>
                            {#if showPassword}
                                <EyeOff/>
                            {:else}
                                <Eye/>
                            {/if}
                        </Button>
                    </InputGroupAddon>
                </InputGroup>

                <div class="flex items-center gap-2">
                    <span class="text-xs text-muted-foreground">Strength: </span>
                    <span class="text-xs font-medium" class:!text-red-600={strength.color === 'red'}
                          class:!text-orange-600={strength.color === 'orange'}
                          class:!text-yellow-600={strength.color === 'yellow'}
                          class:!text-emerald-600={strength.color === 'emerald'}
                          class:!text-green-600={strength.color === 'green'}>{strength.label}</span>
                </div>
                <Progress max={4} value={strength.score}/>
                {#if strength.hints.length}
                    <div class="text-xs text-muted-foreground">Suggestion: {strength.hints.join(", ")}</div>
                {/if}
            </div>

            <div class="grid gap-3">
                <Label class="text-sm font-medium">Confirm password</Label>
                <InputGroup aria-invalid={!!error}>
                    <InputGroupInput type={showConfirm ? "text" : "password"} bind:value={confirmPassword}
                                     autocomplete="new-password" autocapitalize="off" spellcheck={false}
                                     placeholder="Please enter your master password again"/>
                    <InputGroupAddon align="inline-end">
                        <Button variant="ghost" size="icon" onclick={() => (showConfirm = !showConfirm)}
                                aria-label={showConfirm ? "Hide password" : "Show password"}>
                            {#if showConfirm}
                                <EyeOff/>
                            {:else}
                                <Eye/>
                            {/if}
                        </Button>
                    </InputGroupAddon>
                </InputGroup>
            </div>
            <div class="flex items-center space-x-2 gap-2 text-sm">
                <Checkbox id="acknowledged" bind:checked={acknowledged} class="size-4 rounded border"/>
                <Label for="acknowledged">
                    I have saved my master password and it cannot be retrieved if I forget it.
                </Label>
            </div>
            {#if error}
                <Alert variant="destructive">
                    <AlertCircle/>
                    <AlertTitle>The operation failed</AlertTitle>
                    <AlertDescription>{error}</AlertDescription>
                </Alert>
            {/if}
        </CardContent>
        <CardFooter class="flex gap-2">
            <Button class="flex-1" onclick={onInitialize} disabled={loading}>
                {#if loading}
                    <Spinner/>
                {/if}
                Next
            </Button>
        </CardFooter>
    </Card>
</main>

<style lang="scss">
  main {
    &::before {
      position: absolute;
      content: "";
      background: inherit;
      filter: blur(10px);
      transform: scale(1.1);
      z-index: 0;
    }

    &::after {
      position: absolute;
      content: "";
      inset: 0;
      background-color: color-mix(in oklab, var(--color-black) 10%, transparent);
      backdrop-filter: blur(10px);
      z-index: 1;
    }
  }
</style>