export default function randomNumber(from: number, to: number): number {
  return Math.floor(Math.random() * (to - from) + from);
}
