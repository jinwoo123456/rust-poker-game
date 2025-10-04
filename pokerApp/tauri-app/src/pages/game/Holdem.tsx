import { LoadingOverlay } from "@/utils/loading";
import { API_URL } from "@/utils/path";
import axios from "axios";
import { useCallback, useState } from "react";

type HoldemResponse = {
    my_cards?: string[];
    community_cards?: string[];
    message?: string;
};

export default function Holdem() {
    const [myCards, setMyCards] = useState<string[]>([]);
    const [communityCards, setCommunityCards] = useState<string[]>([]);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleDeal = useCallback(async () => {
        setIsLoading(true);
        setError(null);
        try {
            const resp = await axios.post<HoldemResponse>(`${API_URL}/holdem/mycard`, {
                playerCards: myCards,
            });
            const data = resp.data ?? {};
            console.log("홀덤 카드 응답 데이터:", data);

            if (Array.isArray(data.my_cards)) {
                setMyCards(data.my_cards);
            } else {
                setMyCards([]);
            }

            if (Array.isArray(data.community_cards)) {
                setCommunityCards(data.community_cards);
            } else {
                setCommunityCards([]);
            }
        } catch (err) {
            console.error("홀덤 카드 요청 중 오류", err);
            setError("카드를 불러오는 중 문제가 발생했습니다.");
        } finally {
            setIsLoading(false);
        }
    }, [myCards]);

    const handleReset = () => {
        setMyCards([]);
        setCommunityCards([]);
        setError(null);
    };

    return (
        <>
            <LoadingOverlay isLoading={isLoading} />
            <div className="container py-4">
                <div className="d-flex justify-content-between align-items-center mb-3">
                    <h1 className="h3 mb-0">홀덤 카드 받기</h1>
                    <div className="d-flex gap-2">
                        <button className="btn btn-primary" onClick={handleDeal}>
                            카드 받기
                        </button>
                        <button className="btn btn-outline-secondary" onClick={handleReset}>
                            초기화
                        </button>
                    </div>
                </div>

                {error && (
                    <div className="alert alert-danger" role="alert">
                        {error}
                    </div>
                )}

                <div className="row g-4">
                    <div className="col-md-6">
                        <div className="card h-100 shadow-sm">
                            <div className="card-header bg-dark text-white">내 카드</div>
                            <div className="card-body d-flex flex-wrap gap-2">
                                {myCards.length > 0 ? (
                                    myCards.map((card, idx) => (
                                        <span key={idx} className="badge text-bg-primary fs-6 px-3 py-2">
                                            {card}
                                        </span>
                                    ))
                                ) : (
                                    <span className="text-muted">아직 받은 카드가 없습니다.</span>
                                )}
                            </div>
                        </div>
                    </div>
                    <div className="col-md-6">
                        <div className="card h-100 shadow-sm">
                            <div className="card-header bg-secondary text-white">커뮤니티 카드</div>
                            <div className="card-body d-flex flex-wrap gap-2">
                                {communityCards.length > 0 ? (
                                    communityCards.map((card, idx) => (
                                        <span key={idx} className="badge text-bg-warning fs-6 px-3 py-2">
                                            {card}
                                        </span>
                                    ))
                                ) : (
                                    <span className="text-muted">아직 공개된 카드가 없습니다.</span>
                                )}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}

    