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
        error = ""; // 清空错误以便重新校验
    });

    const validate = (): boolean => {
        if (!password || !confirmPassword) {
            error = "请输入主密码并再次确认";
            return false;
        }
        if (password !== confirmPassword) {
            error = "两次输入的密码不一致";
            return false;
        }
        if (strength.score < 3) {
            error = "密码强度过低，请提高复杂度";
            return false;
        }
        if (!acknowledged) {
            error = "请确认已妥善保存主密码";
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
    <Card data-tauri-drag-region class="relative w-full max-w-lg z-[2]">
        <CardHeader>
            <CardTitle class="flex items-center gap-2">
                <Lock class="text-muted-foreground"/>
                The {__NAME__} needs to be initialized
            </CardTitle>
        </CardHeader>
        <CardContent class="grid gap-4">
            <div class="grid gap-3">
                <label class="text-sm font-medium">主密码</label>
                <InputGroup aria-invalid={!!error}>
                    <InputGroupInput type={showPassword ? "text" : "password"} bind:value={password}
                                     autocomplete="new-password" autocapitalize="off" spellcheck={false}
                                     placeholder="请输入主密码（建议12位以上）"/>
                    <InputGroupAddon align="inline-end">
                        <Button variant="ghost" size="icon" onclick={() => (showPassword = !showPassword)}
                                aria-label={showPassword ? "隐藏密码" : "显示密码"}>
                            {#if showPassword}
                                <EyeOff/>
                            {:else}
                                <Eye/>
                            {/if}
                        </Button>
                    </InputGroupAddon>
                </InputGroup>

                <div class="flex items-center gap-2">
                    <span class="text-xs text-muted-foreground">强度：</span>
                    <span class="text-xs font-medium" class:!text-red-600={strength.color === 'red'}
                          class:!text-orange-600={strength.color === 'orange'}
                          class:!text-yellow-600={strength.color === 'yellow'}
                          class:!text-emerald-600={strength.color === 'emerald'}
                          class:!text-green-600={strength.color === 'green'}>{strength.label}</span>
                </div>
                <Progress max={4} value={strength.score}/>
                {#if strength.hints.length}
                    <div class="text-xs text-muted-foreground">建议：{strength.hints.join("，")}</div>
                {/if}
            </div>

            <div class="grid gap-3">
                <label class="text-sm font-medium">确认主密码</label>
                <InputGroup aria-invalid={!!error}>
                    <InputGroupInput type={showConfirm ? "text" : "password"} bind:value={confirmPassword}
                                     autocomplete="new-password" autocapitalize="off" spellcheck={false}
                                     placeholder="请再次输入主密码"/>
                    <InputGroupAddon align="inline-end">
                        <Button variant="ghost" size="icon" onclick={() => (showConfirm = !showConfirm)}
                                aria-label={showConfirm ? "隐藏密码" : "显示密码"}>
                            {#if showConfirm}
                                <EyeOff/>
                            {:else}
                                <Eye/>
                            {/if}
                        </Button>
                    </InputGroupAddon>
                </InputGroup>
            </div>

            <label class="flex items-center gap-2 text-sm">
                <input type="checkbox" bind:checked={acknowledged} class="size-4 rounded border"/>
                我已妥善保存主密码，遗忘将无法找回。
            </label>

            {#if error}
                <Alert variant="destructive">
                    <AlertCircle/>
                    <AlertTitle>操作失败</AlertTitle>
                    <AlertDescription>{error}</AlertDescription>
                </Alert>
            {/if}
        </CardContent>
        <CardFooter class="flex gap-2">
            <Button class="flex-1" onclick={onInitialize} disabled={loading}>
                {#if loading}
                    <Spinner/>
                {/if}
                下一步
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
      background: rgba(255, 255, 255, 0.1);
      backdrop-filter: blur(10px);
      z-index: 1;
    }
  }
</style>