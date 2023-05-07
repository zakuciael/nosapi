import { type ReactNode } from "react";

declare global {
  namespace JSX {
    type ElementType = keyof IntrinsicElements | ((props: any) => Promise<ReactNode> | ReactNode);
  }
}
