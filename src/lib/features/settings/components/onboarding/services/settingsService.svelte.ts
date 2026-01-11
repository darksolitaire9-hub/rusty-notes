import { invoke } from '@tauri-apps/api/core';

export type DeleteBehavior = 'MoveToTrash' | 'Permanent';

export interface Settings {
  version: number;
  notes_folder: string;
  auto_save_interval_secs: number;
  delete_behavior: DeleteBehavior;
  onboarding_completed: boolean;
}

class SettingsService {
  // ========================================================================
  // REACTIVE STATE
  // ========================================================================
  
  settings = $state<Settings | null>(null);
  isLoading = $state(false);
  error = $state<string | null>(null);

  // ========================================================================
  // LOAD SETTINGS
  // ========================================================================
  
  /**
   * Load settings from Rust backend
   * Called once at app startup
   */
  async load() {
    this.isLoading = true;
    this.error = null;

    try {
      this.settings = await invoke<Settings>('get_settings');
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to load settings';
      console.error('Failed to load settings:', err);
    } finally {
      this.isLoading = false;
    }
  }

  /**
   * Update settings
   * Called after onboarding or from settings page
   */
  async update(newSettings: Settings) {
    try {
      await invoke('update_settings', { newSettings });
      this.settings = newSettings; // Update local cache
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to update settings';
      console.error('Failed to update settings:', err);
      throw err;
    }
  }

  /**
   * Get current delete behavior
   * Returns the user's preference from onboarding
   */
  get deleteBehavior(): DeleteBehavior {
    return this.settings?.delete_behavior ?? 'MoveToTrash';
  }

  /**
   * Check if onboarding is completed
   */
  get isOnboardingComplete(): boolean {
    return this.settings?.onboarding_completed ?? false;
  }
}

// Export singleton
export const settingsService = new SettingsService();