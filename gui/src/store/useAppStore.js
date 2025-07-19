import { create } from "zustand";

export const useAppStore = create((set) => ({
  page: "home",
  setPage: (page) => set({ page }),
}));
