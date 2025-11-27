import { LoadingOverlay } from "@/utils/loading";
import { API_URL } from "@/utils/path";
import axios from "axios";
import Cookie from "js-cookie";
import { useCallback, useEffect, useState } from "react";
import { useLocation } from "react-router-dom";

type PlayerSnapshot = {
    card1?: string;
    card2?: string;
    chips?: number;
};

type GameSnapshot = {
    game_id?: string;
    player_id?: string;
    player?: PlayerSnapshot;
    community_cards?: string[];
    blind?: number;
};

type HoldemResponse = {
    success?: number;
    game?: GameSnapshot | null;
    error?: string;
};

export default function Holdem() {
    const [myCards, setMyCards] = useState<string[]>([]);
    const [communityCards, setCommunityCards] = useState<string[]>([]);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [blind, setBlind] = useState<number | null>(null);

    const location = useLocation();
    const userId = Cookie.get("userid");

    const updateFromGame = useCallback((game?: GameSnapshot | null) => {
        if (!game) {
            setMyCards([]);
            setCommunityCards([]);
            setBlind(null);
            return;
        }

        const playerCards = [game.player?.card1, game.player?.card2].filter(
            (card): card is string => typeof card === "string" && card.length > 0,
        );
        setMyCards(playerCards);

        const community = Array.isArray(game.community_cards)
            ? game.community_cards.filter((card): card is string => typeof card === "string" && card.length > 0)
            : [];
        setCommunityCards(community);

        setBlind(typeof game.blind === "number" ? game.blind : null);
    }, []);

    const handleDeal = useCallback(async () => {
        if (!userId) {
            setError("로그인 정보가 없습니다.");
            return;
        }

        setIsLoading(true);
        setError(null);
        try {
            const resp = await axios.get<HoldemResponse>(`${API_URL}/holdem/active/${userId}`);
            const data = resp.data ?? {};
            console.log("홀덤 카드 응답 데이터:", data);

            if (data.success === 1 && data.game) {
                updateFromGame(data.game);
                if (!data.game.player?.card1 && !data.game.player?.card2) {
                    setError("등록된 카드 정보가 없습니다. 매칭을 다시 시도해주세요.");
                }
            } else {
                updateFromGame(null);
                setError(data.error ?? "게임 정보가 없습니다. 매칭을 다시 시도해주세요.");
            }
        } catch (err) {
            console.error("홀덤 카드 요청 중 오류", err);
            setError("카드를 불러오는 중 문제가 발생했습니다.");
        } finally {
            setIsLoading(false);
        }
    }, [updateFromGame, userId]);

    useEffect(() => {
        const state = location.state as unknown;
        if (!state || typeof state !== "object") {
            return;
        }

        const stateRecord = state as Record<string, unknown>;
        const maybeGame = (stateRecord.game ?? state) as GameSnapshot | null;

        if (maybeGame && (maybeGame.community_cards || maybeGame.player)) {
            updateFromGame(maybeGame);
        }
    }, [location.state, updateFromGame]);

    useEffect(() => {
        if (!userId) {
            setError("로그인 정보가 없습니다.");
            return;
        }

        handleDeal();
    }, [handleDeal, userId]);

    const handleReset = () => {
        setMyCards([]);
        setCommunityCards([]);
        setError(null);
        setBlind(null);
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

                {blind !== null && (
                    <div className="alert alert-info" role="status">
                        현재 블라인드: {blind.toLocaleString()} chips
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

    