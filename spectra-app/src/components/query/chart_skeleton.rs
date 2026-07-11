use leptos::prelude::*;
use orbital::primitives::{Skeleton, SkeletonItem};

#[component]
pub fn ChartSkeleton() -> impl IntoView {
    view! {
        <Skeleton>
            <SkeletonItem />
        </Skeleton>
    }
}
