import * as React from 'react'

import { cn } from '@/lib/utils'

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {}

const NumberInput = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, ...props }, ref) => {
    return (
      <div className="dark:bg-neutral-600 p-1">
        <input
          type="number"
          className={cn(
            'flex w-full border border-input px-1 text-md ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-neutral-700',
            className,
          )}
          ref={ref}
          {...props}
        />
      </div>
    )
  },
)
NumberInput.displayName = 'NumberInput'

export { NumberInput }
