import { FormEvent } from 'react'
import { cn } from '@/lib/utils'

export interface ButtonWithIconProps {
  className?: string
  text: string
  type: 'submit' | 'button' | 'reset'
  icon: string
  color?: 'success' | 'info' | 'warn'
  onClick?: () => void
}

const ButtonWithIcon = ({
  className,
  text,
  type,
  icon,
  color,
  onClick,
}: ButtonWithIconProps) => {
  const colorClasses = {
    success: 'dark:bg-green-900',
    info: 'dark:bg-blue-900',
    warn: 'dark:bg-red-900',
    default: 'dark:bg-blue-900',
  }
  const colorName = colorClasses[color ? color : 'default']

  return (
    <button
      className={cn('px-2 rounded-full', colorName, className)}
      type={type}
      onClick={
        onClick
          ? (e: FormEvent) => {
              e.stopPropagation()
              e.preventDefault()
              onClick()
            }
          : undefined
      }
    >
      <div className="flex items-center">
        <span className={`${icon} mr-1`}></span>
        <span>{text}</span>
      </div>
    </button>
  )
}

export default ButtonWithIcon
