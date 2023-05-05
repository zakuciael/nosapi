declare global {
  namespace JSX {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    type ElementType = keyof IntrinsicElements | ((props: any) => Promise<ReactNode> | ReactNode);
  }
}
