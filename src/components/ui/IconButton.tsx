import { ComponentProps } from 'react'
import { twMerge } from 'tailwind-merge'

export type IconButtonProps = {
  icon: string
  onClick: () => void
} & ComponentProps<'button'>

const IconButton = ({ icon, onClick, className }: IconButtonProps) => {
  const baseClass = `${icon} cursor-pointer p-0`
  const mergedClass = twMerge(baseClass, className)

  return (
    <button
      className={mergedClass}
      onClick={(e: any) => {
        e.stopPropagation()
        e.preventDefault()
        onClick()
      }}
    ></button>
  )
}

export default IconButton
