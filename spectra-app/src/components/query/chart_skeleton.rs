use leptos::prelude::*;
use orbital::primitives::{Skeleton, SkeletonItem};

#[component]
pub fn ChartSkeleton() -> impl IntoView {
    view! {
        <Skeleton>
            <SkeletonItem width="100%".to_string() height="12rem".to_string() />
        </Skeleton>
    }
}
