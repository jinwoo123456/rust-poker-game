import { type FC } from "react";
import { ClipLoader } from "react-spinners";
import "@/styles/components/loading-overlay.css";

interface LoadingOverlayProps {
    isLoading: boolean;
    color?: string;
    size?: number;
}

export const LoadingOverlay: FC<LoadingOverlayProps> = ({
    isLoading,
    color = '#36d7b7',
    size = 100,
}) => {
    if (!isLoading) {
        return null;
    }

    return (
        <div className="loading-overlay">
            <div className="loading-overlay__spinner">
                <ClipLoader color={color} size={size} />
            </div>
        </div>
    );
};
